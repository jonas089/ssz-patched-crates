// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::operations::ExecutionPayloadTestCase;
use ethereum_consensus::bellatrix::minimal as spec;

#[test]
fn test_bad_everything_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_everything_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_execution_first_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_execution_first_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_execution_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_execution_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_parent_hash_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_parent_hash_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_random_first_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_random_first_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_random_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_random_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_timestamp_first_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_timestamp_first_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_bad_timestamp_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/bad_timestamp_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_non_empty_extra_data_first_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/non_empty_extra_data_first_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_non_empty_extra_data_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/non_empty_extra_data_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_success_first_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_first_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_success_first_payload_with_gap_slot() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_first_payload_with_gap_slot");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_success_regular_payload() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_regular_payload");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}

#[test]
fn test_success_regular_payload_with_gap_slot() {
    let mut test_case = ExecutionPayloadTestCase::<spec::BeaconState, spec::ExecutionPayload>::from("../consensus-spec-tests/tests/minimal/bellatrix/operations/execution_payload/pyspec_tests/success_regular_payload_with_gap_slot");

    test_case.execute(|state, operation, context, execution_valid| {
        let execution_engine = spec::DefaultExecutionEngine::new(execution_valid);
        spec::process_execution_payload(state, operation, &execution_engine, context)
    });
}
