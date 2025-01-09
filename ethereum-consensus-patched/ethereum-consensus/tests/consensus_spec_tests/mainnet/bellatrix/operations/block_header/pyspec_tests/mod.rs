// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::BlockHeaderTestCase;
use ethereum_consensus::bellatrix::mainnet as spec;

#[test]
fn test_invalid_multiple_blocks_single_slot() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_multiple_blocks_single_slot");

    test_case.execute(spec::process_block_header);
}

#[test]
fn test_invalid_parent_root() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_parent_root");

    test_case.execute(spec::process_block_header);
}

#[test]
fn test_invalid_proposer_index() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_proposer_index");

    test_case.execute(spec::process_block_header);
}

#[test]
fn test_invalid_slot_block_header() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/invalid_slot_block_header");

    test_case.execute(spec::process_block_header);
}

#[test]
fn test_proposer_slashed() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/proposer_slashed");

    test_case.execute(spec::process_block_header);
}

#[test]
fn test_success_block_header() {
    let mut test_case = BlockHeaderTestCase::<spec::BeaconState, spec::BeaconBlock>::from("../consensus-spec-tests/tests/mainnet/bellatrix/operations/block_header/pyspec_tests/success_block_header");

    test_case.execute(spec::process_block_header);
}
