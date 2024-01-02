/// A savings account
pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount { balance: 0 }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            panic!("Can not deposit a negative amount!");
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}
