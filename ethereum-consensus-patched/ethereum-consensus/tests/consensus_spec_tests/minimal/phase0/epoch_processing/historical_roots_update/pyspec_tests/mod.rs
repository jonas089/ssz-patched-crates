// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::HistoricalRootsUpdateTestCase;
use ethereum_consensus::phase0::minimal as spec;

#[test]
fn test_historical_root_accumulator() {
    let mut test_case = HistoricalRootsUpdateTestCase::<spec::BeaconState>::from("../consensus-spec-tests/tests/minimal/phase0/epoch_processing/historical_roots_update/pyspec_tests/historical_root_accumulator");

    test_case.execute(spec::process_historical_roots_update);
}
