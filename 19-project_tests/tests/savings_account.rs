// integration tests are located under tests folder
// as it is external to the main project it is necessary to import the module to test
use project_tests::SavingsAccount;

// add common functionality
mod utils;

#[test]
fn should_have_a_starting_balance_of_0() {
    utils::common_setup(); // perform a test setup

    let account = SavingsAccount::new();

    assert_eq!(account.get_balance(), 0);
}