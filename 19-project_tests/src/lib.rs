/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0.
    /// Sections can be created below. Common ones are Examples, Panic, or Failure
    /// 
    /// # Examples
    /// 
    /// ```
    /// use project_tests::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount {
            balance: 0
        }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Cannot deposit a negatve amount"); // a Result::Error should be returned instead!
        }

        self.balance += amount
    }

    pub fn transfer(&mut self, account_number: i32, amount: i32) -> Result<String, String> {
        if self.balance < amount {
            return Err(String::from("There is not enough balance"));
        }

        self.balance -= amount;

        Ok(format!("Transferred ${amount} to ${account_number}"))
    }
}

// tests are located in a module called tests configured to be compiled only when test feature is enabled (when runnning cargo test)
// could be inline (like here) or in a separate file but
#[cfg(test)] 
mod tests {
    // import all components to test
    use super::*;

    #[test] // this is a test
    fn should_have_a_starting_balance_of_0() {
        let account = SavingsAccount::new();

        assert_eq!(account.get_balance(), 0);
    }

    #[test] // this is a test
    fn should_be_able_to_deposit() {
        let mut account = SavingsAccount::new();

        account.deposit(120);

        assert_eq!(account.get_balance(), 120); // equals
        assert_ne!(account.get_balance(), 0); // not equals
        assert!(account.get_balance() == 120, "Balance was not correct"); // compare with custom error message
    }

    #[test] // this is a test
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();

        account.deposit(-10);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> { // Ok means the test passes, Error it does not
        let mut account = SavingsAccount::new();

        account.deposit(100);
        account.transfer(123456, 50)?; // ? propagates an error

        Ok(())
    }

    #[test]
    fn should_not_be_able_to_transfer_money() {
        let mut account = SavingsAccount::new();

        account.deposit(100);

        let result = account.transfer(123456, 150);

        assert!(result.is_err());
    }
}