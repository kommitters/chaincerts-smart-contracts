use crate::test::setup::VCsContractTest;

#[test]
fn test_initialize_with_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &amount);
}

#[test]
fn test_initialize_without_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &None);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_initialize_with_too_high_amount() {
    let VCsContractTest {
        env: _env,
        admin,
        amount: _,
        contract,
    } = VCsContractTest::setup();
    let high_amount = Some(101);

    contract.initialize(&admin, &high_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_an_already_initialized_contract() {
    let VCsContractTest {
        env: _env,
        admin,
        amount,
        contract,
    } = VCsContractTest::setup();

    contract.initialize(&admin, &amount);
    contract.initialize(&admin, &amount);
}
