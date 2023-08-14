#[derive(Clone)]
pub struct Account {
    account_number: String,
    balance: f64,
}

impl Account {
    // Constructor to create a new Account
    pub fn new(account_number: String, balance: f64) -> Self {
        Account {
            account_number,
            balance,
        }
    }

    // Method to deposit money into the account
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${} into account {}", amount, self.account_number);
    }

    // Method to withdraw money from the account
    pub fn withdraw(&mut self, amount: f64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew ${} from account {}", amount, self.account_number);
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    // Method to transfer money to another account
    pub fn transfer(&mut self, to_account: &mut Account, amount: f64) -> Result<(), &'static str> {
        if self.balance >= amount {
            self.balance -= amount;
            to_account.balance += amount;
            println!("Transferred ${} from account {} to account {}", amount, self.account_number, to_account.account_number);
            Ok(())
        } else {
            Err("Insufficient funds for transfer")
        }
    }

    // Method to get the current balance of the account
    pub fn get_balance(&self) -> f64 {
        self.balance
    }

    // Method to get the account number
    pub fn get_account_number(&self) -> &str {
        &self.account_number
    }

    // implement clone for Account
    

}

// Path: bank/src/main.rs
