// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::ssz_static::Eth1DataTestCase;
use ethereum_consensus::{phase0::minimal as spec, ssz::prelude::*};

#[test]
fn test_case_0() {
    let test_case = Eth1DataTestCase::from(
        "../consensus-spec-tests/tests/minimal/phase0/ssz_static/Eth1Data/ssz_max/case_0",
    );

    test_case.execute(|encoding| {
        let mut data: spec::Eth1Data =
            ethereum_consensus::ssz::prelude::deserialize(encoding).unwrap();
        let serialized = ethereum_consensus::ssz::prelude::serialize(&data).unwrap();
        let root = data.hash_tree_root().unwrap();
        (serialized, root)
    });
}
