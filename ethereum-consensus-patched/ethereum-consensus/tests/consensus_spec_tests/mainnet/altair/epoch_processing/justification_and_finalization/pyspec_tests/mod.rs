// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::JustificationAndFinalizationTestCase;
use ethereum_consensus::altair::mainnet as spec;

#[test]
fn test_123_ok_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/123_ok_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_123_poor_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/123_poor_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_12_ok_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/12_ok_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_12_ok_support_messed_target() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/12_ok_support_messed_target");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_12_poor_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/12_poor_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_234_ok_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/234_ok_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_234_poor_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/234_poor_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_23_ok_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/23_ok_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_23_poor_support() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/23_poor_support");

    test_case.execute(spec::process_justification_and_finalization);
}

#[test]
fn test_balance_threshold_with_exited_validators() {
    let mut test_case = JustificationAndFinalizationTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/mainnet/altair/epoch_processing/justification_and_finalization/pyspec_tests/balance_threshold_with_exited_validators");

    test_case.execute(spec::process_justification_and_finalization);
}
