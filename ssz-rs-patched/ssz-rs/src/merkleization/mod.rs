mod node;
mod proofs;

use crate::{
    lib::*,
    ser::{Serialize, SerializeError},
};
use sha2::{Digest, Sha256};

pub use node::Node;
pub use proofs::is_valid_merkle_branch;

pub(crate) const BYTES_PER_CHUNK: usize = 32;
pub(crate) const BITS_PER_CHUNK: usize = BYTES_PER_CHUNK * (crate::BITS_PER_BYTE as usize);

/// A `Merkleized` type provides a "hash tree root" following the SSZ spec.
pub trait Merkleized {
    /// Compute the "hash tree root" of `Self`.
    fn hash_tree_root(&mut self) -> Result<Node, MerkleizationError>;

    /// Indicate the "composite" nature of `Self`.
    fn is_composite_type() -> bool {
        true
    }
}

/// An error encountered during merkleization.
#[derive(Debug)]
pub enum MerkleizationError {
    /// An error serializing a type while computing the hash tree.
    SerializationError(SerializeError),
    /// More data was provided than expected
    InputExceedsLimit(usize),
    /// Proof verification failed
    InvalidProof,
}

impl From<SerializeError> for MerkleizationError {
    fn from(err: SerializeError) -> Self {
        MerkleizationError::SerializationError(err)
    }
}

impl Display for MerkleizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializationError(err) => {
                write!(f, "failed to serialize value: {err}")
            }
            Self::InputExceedsLimit(size) => write!(f, "data exceeds the declared limit {size}"),
            Self::InvalidProof => write!(f, "merkle proof verification failed"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MerkleizationError {}

// Ensures `buffer` can be exactly broken up into `BYTES_PER_CHUNK` chunks of bytes
// via padding any partial chunks at the end of `buffer`
pub fn pack_bytes(buffer: &mut Vec<u8>) {
    let incomplete_chunk_len = buffer.len() % BYTES_PER_CHUNK;
    if incomplete_chunk_len != 0 {
        // SAFETY: checked subtraction is unnecessary,
        // as BYTES_PER_CHUNK > incomplete_chunk_len; qed
        let bytes_to_pad = BYTES_PER_CHUNK - incomplete_chunk_len;
        buffer.resize(buffer.len() + bytes_to_pad, 0);
    }
}

// Packs serializations of `values` into the return buffer with the
// guarantee that `buffer.len() % BYTES_PER_CHUNK == 0`
pub fn pack<T>(values: &[T]) -> Result<Vec<u8>, MerkleizationError>
where
    T: Serialize,
{
    let mut buffer = vec![];
    for value in values {
        value.serialize(&mut buffer)?;
    }
    pack_bytes(&mut buffer);
    Ok(buffer)
}

fn hash_nodes(hasher: &mut Sha256, a: &[u8], b: &[u8], out: &mut [u8]) {
    hasher.update(a);
    hasher.update(b);
    out.copy_from_slice(&hasher.finalize_reset());
}

const MAX_MERKLE_TREE_DEPTH: usize = 64;

#[derive(Debug)]
struct Context {
    zero_hashes: [u8; MAX_MERKLE_TREE_DEPTH * BYTES_PER_CHUNK],
}

impl Index<usize> for Context {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.zero_hashes[index * BYTES_PER_CHUNK..(index + 1) * BYTES_PER_CHUNK]
    }
}

// Grab the precomputed context from the build stage
include!(concat!(env!("OUT_DIR"), "/context.rs"));

/// Return the root of the Merklization of a binary tree formed from `chunks`.
///
/// `chunks` forms the bottom layer of a binary tree that is Merkleized.
///
/// This implementation is memory efficient by relying on pre-computed subtrees of all
/// "zero" leaves stored in the `CONTEXT`. SSZ specifies that `chunks` is padded to the next power
/// of two and this can be quite large for some types. "Zero" subtrees are virtualized to avoid the
/// memory and computation cost of large trees with partially empty leaves.
///
/// The implementation approach treats `chunks` as the bottom layer of a perfect binary tree
/// and for each height performs the hashing required to compute the parent layer in place.
/// This process is repated until the root is computed.
///
/// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
/// Invariant: `leaf_count.next_power_of_two() == leaf_count`
/// Invariant: `leaf_count != 0`
/// Invariant: `leaf_count.trailing_zeros() < MAX_MERKLE_TREE_DEPTH`
fn merkleize_chunks_with_virtual_padding(
    chunks: &[u8],
    leaf_count: usize,
) -> Result<Node, MerkleizationError> {
    debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
    // NOTE: This also asserts that leaf_count != 0
    debug_assert!(leaf_count.next_power_of_two() == leaf_count);
    // SAFETY: this holds as long as leaf_count != 0 and usize is no longer than u64
    debug_assert!((leaf_count.trailing_zeros() as usize) < MAX_MERKLE_TREE_DEPTH);

    let chunk_count = chunks.len() / BYTES_PER_CHUNK;
    let height = leaf_count.trailing_zeros() + 1;

    if chunk_count == 0 {
        // SAFETY: checked subtraction is unnecessary, as height >= 1; qed
        let depth = height - 1;
        // SAFETY: index is safe while depth == leaf_count.trailing_zeros() < MAX_MERKLE_TREE_DEPTH;
        // qed
        return Ok(CONTEXT[depth as usize].try_into().expect("can produce a single root chunk"))
    }

    let mut layer = chunks.to_vec();
    // SAFETY: checked subtraction is unnecessary, as we return early when chunk_count == 0; qed
    let mut last_index = chunk_count - 1;
    let mut hasher = Sha256::new();
    // for each layer of the tree, starting from the bottom and walking up to the root:
    for k in (1..height).rev() {
        // for each pair of nodes in this layer:
        for i in (0..2usize.pow(k)).step_by(2) {
            let parent_index = i / 2;
            let (parent, left, right) = match i.cmp(&last_index) {
                Ordering::Less => {
                    // SAFETY: index is safe because (i+1)*BYTES_PER_CHUNK < layer.len():
                    // i < last_index == chunk_count - 1 == (layer.len() / BYTES_PER_CHUNK) - 1
                    // so i+1 < layer.len() / BYTES_PER_CHUNK
                    // so (i+1)*BYTES_PER_CHUNK < layer.len(); qed
                    let focus =
                        &mut layer[parent_index * BYTES_PER_CHUNK..(i + 2) * BYTES_PER_CHUNK];
                    // SAFETY: checked subtraction is unnecessary:
                    // focus.len() = (i + 2 - parent_index) * BYTES_PER_CHUNK
                    // and
                    // i >= parent_index
                    // so focus.len() >= 2 * BYTES_PER_CHUNK; qed
                    let children_index = focus.len() - 2 * BYTES_PER_CHUNK;
                    let (parent, children) = focus.split_at_mut(children_index);
                    let (left, right) = children.split_at_mut(BYTES_PER_CHUNK);

                    // NOTE: we do not need mutability on `right` here so drop that capability
                    (parent, left, &*right)
                }
                Ordering::Equal => {
                    // SAFETY: index is safe because i*BYTES_PER_CHUNK < layer.len():
                    // i*BYTES_PER_CHUNK < (i+1)*BYTES_PER_CHUNK < layer.len()
                    // (see previous case); qed
                    let focus =
                        &mut layer[parent_index * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK];
                    // SAFETY: checked subtraction is unnecessary:
                    // focus.len() = (i + 1 - parent_index) * BYTES_PER_CHUNK
                    // and
                    // i >= parent_index
                    // so focus.len() >= BYTES_PER_CHUNK; qed
                    let children_index = focus.len() - BYTES_PER_CHUNK;
                    // NOTE: left.len() == BYTES_PER_CHUNK
                    let (parent, left) = focus.split_at_mut(children_index);
                    // SAFETY: checked subtraction is unnecessary:
                    // k <= height - 1
                    // so depth >= height - (height - 1) - 1
                    //           = 0; qed
                    let depth = height - k - 1;
                    // SAFETY: index is safe because depth < CONTEXT.len():
                    // depth <= height - 1 == leaf_count.trailing_zeros()
                    // leaf_count.trailing_zeros() < MAX_MERKLE_TREE_DEPTH == CONTEXT.len(); qed
                    let right = &CONTEXT[depth as usize];
                    (parent, left, right)
                }
                _ => break,
            };
            if i == 0 {
                // NOTE: nodes share memory here and so we can't use the `hash_nodes` utility
                // as the disjunct nature is reflect in that functions type signature
                // so instead we will just replicate here.
                hasher.update(&left);
                hasher.update(right);
                left.copy_from_slice(&hasher.finalize_reset());
            } else {
                // SAFETY: index is safe because parent.len() % BYTES_PER_CHUNK == 0 and
                // parent isn't empty; qed
                hash_nodes(&mut hasher, left, right, &mut parent[..BYTES_PER_CHUNK]);
            }
        }
        last_index /= 2;
    }

    // SAFETY: index is safe because layer.len() >= BYTES_PER_CHUNK:
    // layer.len() == chunks.len()
    // chunks.len() % BYTES_PER_CHUNK == 0 and chunks.len() != 0 (because chunk_count != 0)
    // so chunks.len() >= BYTES_PER_CHUNK; qed
    Ok(layer[..BYTES_PER_CHUNK].try_into().expect("can produce a single root chunk"))
}

// Return the root of the Merklization of a binary tree formed from `chunks`.
// Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
pub fn merkleize(chunks: &[u8], limit: Option<usize>) -> Result<Node, MerkleizationError> {
    debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
    let chunk_count = chunks.len() / BYTES_PER_CHUNK;
    let mut leaf_count = chunk_count.next_power_of_two();
    if let Some(limit) = limit {
        if limit < chunk_count {
            return Err(MerkleizationError::InputExceedsLimit(limit))
        }
        leaf_count = limit.next_power_of_two();
    }
    merkleize_chunks_with_virtual_padding(chunks, leaf_count)
}

fn mix_in_decoration(root: &Node, mut decoration: usize) -> Node {
    let decoration_data = decoration.hash_tree_root().expect("can merkleize usize");

    let mut hasher = Sha256::new();
    let mut output = vec![0u8; BYTES_PER_CHUNK];
    hash_nodes(&mut hasher, root.as_ref(), decoration_data.as_ref(), &mut output);
    output.as_slice().try_into().expect("can extract root")
}

pub(crate) fn mix_in_length(root: &Node, length: usize) -> Node {
    mix_in_decoration(root, length)
}

pub fn mix_in_selector(root: &Node, selector: usize) -> Node {
    mix_in_decoration(root, selector)
}

pub(crate) fn elements_to_chunks<'a, T: Merkleized + 'a>(
    elements: impl Iterator<Item = (usize, &'a mut T)>,
    count: usize,
) -> Result<Vec<u8>, MerkleizationError> {
    let mut chunks = vec![0u8; count * BYTES_PER_CHUNK];
    for (i, elem) in elements {
        let chunk = elem.hash_tree_root()?;
        let range = i * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK;
        chunks[range].copy_from_slice(chunk.as_ref());
    }
    Ok(chunks)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate as ssz_rs;
    use crate::prelude::*;

    macro_rules! hex {
        ($input:expr) => {
            hex::decode($input).unwrap()
        };
    }

    #[test]
    fn test_packing_basic_types_simple() {
        let b = true;
        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        let input = &[b];
        let result = pack(input).expect("can pack values");
        assert!(result.len() == BYTES_PER_CHUNK);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_packing_basic_types_extended() {
        let b = true;
        let input = &[b, !b, !b, b];
        let result = pack(input).expect("can pack values");

        let mut expected = vec![0u8; BYTES_PER_CHUNK];
        expected[0] = 1u8;
        expected[3] = 1u8;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_packing_basic_types_multiple() {
        let data = U256::from_le_bytes([1u8; 32]);
        let input = &[data, data, data];
        let result = pack(input).expect("can pack values");

        let expected = vec![1u8; 3 * 32];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_merkleize_basic() {
        let input = &[];
        let result = merkleize(input, None).expect("can merkle");
        assert_eq!(result, Node::default());

        let b = true;
        let input = &[b];
        let input = pack(input).expect("can pack");
        let result = merkleize(&input, None).expect("can merkle");
        let mut expected = Node::default();
        expected.as_mut()[0] = 1u8;
        assert_eq!(result, expected);
    }

    // Return the root of the Merklization of a binary tree formed from `chunks`.
    // Invariant: `chunks.len() % BYTES_PER_CHUNK == 0`
    // Invariant: `leaf_count.next_power_of_two() == leaf_count`
    // NOTE: naive implementation, can make much more efficient
    fn merkleize_chunks(chunks: &[u8], leaf_count: usize) -> Result<Node, MerkleizationError> {
        debug_assert!(chunks.len() % BYTES_PER_CHUNK == 0);
        debug_assert!(leaf_count.next_power_of_two() == leaf_count);

        // SAFETY: checked subtraction is unnecessary,
        // as leaf_count != 0 (0.next_power_of_two() == 1); qed
        let node_count = 2 * leaf_count - 1;
        // SAFETY: checked subtraction is unnecessary, as node_count >= leaf_count; qed
        let interior_count = node_count - leaf_count;
        let leaf_start = interior_count * BYTES_PER_CHUNK;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; node_count * BYTES_PER_CHUNK];
        buffer[leaf_start..leaf_start + chunks.len()].copy_from_slice(chunks);

        for i in (1..node_count).rev().step_by(2) {
            // SAFETY: checked subtraction is unnecessary, as i >= 1; qed
            let parent_index = (i - 1) / 2;
            let focus = &mut buffer[parent_index * BYTES_PER_CHUNK..(i + 1) * BYTES_PER_CHUNK];
            // SAFETY: checked subtraction is unnecessary:
            // focus.len() = (i + 1 - parent_index) * BYTES_PER_CHUNK
            //             = ((2*i + 2 - i + 1) / 2) * BYTES_PER_CHUNK
            //             = ((i + 3) / 2) * BYTES_PER_CHUNK
            // and
            // i >= 1
            // so focus.len() >= 2 * BYTES_PER_CHUNK; qed
            let children_index = focus.len() - 2 * BYTES_PER_CHUNK;
            // NOTE: children.len() == 2 * BYTES_PER_CHUNK
            let (parent, children) = focus.split_at_mut(children_index);
            let (left, right) = children.split_at(BYTES_PER_CHUNK);
            hash_nodes(&mut hasher, left, right, &mut parent[..BYTES_PER_CHUNK]);
        }
        Ok(buffer[..BYTES_PER_CHUNK].try_into().expect("can produce a single root chunk"))
    }

    #[test]
    fn test_naive_merkleize_chunks() {
        let chunks = vec![0u8; 2 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 2).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("f5a5fd42d16a20302798ef6ed309979b43003d2320d9f0e8ea9831a92759fb4b")
        );

        let chunks = vec![1u8; 2 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 2).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("7c8975e1e60a5c8337f28edf8c33c3b180360b7279644a9bc1af3c51e6220bf5")
        );

        let chunks = vec![0u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 4).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("db56114e00fdd4c1f85c892bf35ac9a89289aaecb1ebd0a96cde606a748b5d71")
        );

        let chunks = vec![1u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 4).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("29797eded0e83376b70f2bf034cc0811ae7f1414653b1d720dfd18f74cf13309")
        );

        let chunks = vec![2u8; BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 8).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("fa4cf775712aa8a2fe5dcb5a517d19b2e9effcf58ff311b9fd8e4a7d308e6d00")
        );

        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root = merkleize_chunks(&chunks, 8).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("0ae67e34cba4ad2bbfea5dc39e6679b444021522d861fab00f05063c54341289")
        );
    }

    #[test]
    fn test_merkleize_chunks() {
        let chunks = vec![1u8; 3 * BYTES_PER_CHUNK];
        let root = merkleize_chunks_with_virtual_padding(&chunks, 4).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("65aa94f2b59e517abd400cab655f42821374e433e41b8fe599f6bb15484adcec")
        );

        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root = merkleize_chunks_with_virtual_padding(&chunks, 8).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("0ae67e34cba4ad2bbfea5dc39e6679b444021522d861fab00f05063c54341289")
        );

        let chunks = vec![1u8; 6 * BYTES_PER_CHUNK];
        let root = merkleize_chunks_with_virtual_padding(&chunks, 8).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("0ef7df63c204ef203d76145627b8083c49aa7c55ebdee2967556f55a4f65a238")
        );
    }

    #[test]
    fn test_merkleize_chunks_with_many_virtual_nodes() {
        let chunks = vec![1u8; 5 * BYTES_PER_CHUNK];
        let root =
            merkleize_chunks_with_virtual_padding(&chunks, 2usize.pow(10)).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("2647cb9e26bd83eeb0982814b2ac4d6cc4a65d0d98637f1a73a4c06d3db0e6ce")
        );

        let chunks = vec![1u8; 70 * BYTES_PER_CHUNK];
        let root =
            merkleize_chunks_with_virtual_padding(&chunks, 2usize.pow(63)).expect("can merkleize");
        assert_eq!(
            root.as_ref(),
            hex!("9317695d95b5a3b46e976b5a9cbfcfccb600accaddeda9ac867cc9669b862979")
        );
    }

    #[test]
    fn test_hash_tree_root_of_list() {
        let mut a_list = List::<u16, 1024>::try_from(vec![
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535,
            65535, 65535, 65535, 65535,
        ])
        .unwrap();
        let root = a_list.hash_tree_root().expect("can compute root");
        assert_eq!(
            root.as_ref(),
            hex!("d20d2246e1438d88de46f6f41c7b041f92b673845e51f2de93b944bf599e63b1")
        );
    }

    #[test]
    fn test_hash_tree_root_of_empty_list() {
        let mut a_list = List::<u16, 1024>::try_from(vec![]).unwrap();
        let root = a_list.hash_tree_root().expect("can compute root");
        assert_eq!(
            root.as_ref(),
            hex!("c9eece3e14d3c3db45c38bbf69a4cb7464981e2506d8424a0ba450dad9b9af30")
        );
    }

    #[test]
    fn test_hash_tree_root() {
        #[derive(PartialEq, Eq, Debug, SimpleSerialize, Clone)]
        enum Bar {
            A(u32),
            B(List<bool, 32>),
        }

        impl Default for Bar {
            fn default() -> Self {
                Self::A(Default::default())
            }
        }

        #[derive(PartialEq, Eq, Debug, Default, SimpleSerialize, Clone)]
        struct Foo {
            a: u32,
            b: Vector<u32, 4>,
            c: bool,
            d: Bitlist<27>,
            e: Bar,
            f: Bitvector<4>,
            g: List<u16, 7>,
        }

        let mut foo = Foo {
            a: 16u32,
            b: Vector::try_from(vec![3u32, 2u32, 1u32, 10u32]).unwrap(),
            c: true,
            d: Bitlist::try_from(
                [
                    true, false, false, true, true, false, true, false, true, true, false, false,
                    true, true, false, true, false, true, true, false, false, true, true, false,
                    true, false, true,
                ]
                .as_ref(),
            )
            .unwrap(),
            e: Bar::B(List::try_from(vec![true, true, false, false, false, true]).unwrap()),
            f: Bitvector::try_from([false, true, false, true].as_ref()).unwrap(),
            g: List::try_from(vec![1, 2]).unwrap(),
        };

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root.as_ref(),
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let mut original_foo = foo.clone();

        foo.b[2] = 44u32;
        foo.d.pop();
        foo.e = Bar::A(33);

        let root = original_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root.as_ref(),
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root.as_ref(),
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );

        let encoding = serialize(&original_foo).expect("can serialize");

        let mut restored_foo = Foo::deserialize(&encoding).expect("can deserialize");

        let root = restored_foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root.as_ref(),
            hex!("7078155bf8f0dc42d8afccec8d9b5aeb54f0a2e8e58fcef3e723f6a867232ce7")
        );

        restored_foo.b[2] = 44u32;
        restored_foo.d.pop();
        restored_foo.e = Bar::A(33);

        let root = foo.hash_tree_root().expect("can make root");
        assert_eq!(
            root.as_ref(),
            hex!("0063bfcfabbca567483a2ee859fcfafb958329489eb328ac7f07790c7df1b231")
        );
    }

    #[test]
    fn test_simple_serialize_of_root() {
        let mut root = Node::default();
        let mut result = vec![];
        let _ = root.serialize(&mut result).expect("can encode");
        let expected_encoding = vec![0; 32];
        assert_eq!(result, expected_encoding);

        let recovered_root = Node::deserialize(&result).expect("can decode");
        assert_eq!(recovered_root, Node::default());

        let hash_tree_root = root.hash_tree_root().expect("can find root");
        assert_eq!(hash_tree_root, Node::default());
    }

    #[test]
    fn test_derive_merkleized() {
        #[derive(Debug, Merkleized)]
        struct Foo {
            a: U256,
        }

        let mut foo = Foo { a: U256::from(68) };
        let foo_root = foo.hash_tree_root().unwrap();
        let expected_root =
            hex::decode("4400000000000000000000000000000000000000000000000000000000000000")
                .unwrap();
        assert_eq!(foo_root.as_ref(), expected_root);
    }
}
