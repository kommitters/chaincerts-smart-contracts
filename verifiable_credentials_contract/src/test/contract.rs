use crate::test::setup::VCsContractTest;
// use soroban_sdk::testutils::Address as _;

#[test]
fn test_initialize() {
    let VCsContractTest {
        env: _env,
        admin,
        contract,
    } = VCsContractTest::setup();
    contract.initialize(&admin, &Some(10));
}
