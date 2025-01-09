// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::ProposerSlashingTestCase;
use ethereum_consensus::{phase0::mainnet as spec, ssz::prelude::*};

#[test]
fn test_case_0() {
    let  test_case = ProposerSlashingTestCase::<>::from("../consensus-spec-tests/tests/mainnet/phase0/ssz_static/ProposerSlashing/ssz_random/case_0");

    test_case.execute(|encoding| {
        let mut data: spec::ProposerSlashing =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_1() {
    let  test_case = ProposerSlashingTestCase::<>::from("../consensus-spec-tests/tests/mainnet/phase0/ssz_static/ProposerSlashing/ssz_random/case_1");

    test_case.execute(|encoding| {
        let mut data: spec::ProposerSlashing =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_2() {
    let  test_case = ProposerSlashingTestCase::<>::from("../consensus-spec-tests/tests/mainnet/phase0/ssz_static/ProposerSlashing/ssz_random/case_2");

    test_case.execute(|encoding| {
        let mut data: spec::ProposerSlashing =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_3() {
    let  test_case = ProposerSlashingTestCase::<>::from("../consensus-spec-tests/tests/mainnet/phase0/ssz_static/ProposerSlashing/ssz_random/case_3");

    test_case.execute(|encoding| {
        let mut data: spec::ProposerSlashing =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}

#[test]
fn test_case_4() {
    let  test_case = ProposerSlashingTestCase::<>::from("../consensus-spec-tests/tests/mainnet/phase0/ssz_static/ProposerSlashing/ssz_random/case_4");

    test_case.execute(|encoding| {
        let mut data: spec::ProposerSlashing =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
