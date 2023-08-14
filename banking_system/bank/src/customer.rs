
use crate::bank::Bank;
use crate::account::Account;
#[derive(Clone)]
pub struct Customer {
    name: String,
    address: String,
    phone_number: String,
    accounts: Vec<Account>,
}

impl Customer {
    // Constructor to create a new Customer
    pub fn new(name: String, address: String, phone_number: String) -> Self {
        Customer {
            name,
            address,
            phone_number,
            accounts: Vec::new(),
        }
    }

    // Method to open a new account for the customer
    pub fn open_account(&mut self, bank: &mut Bank, initial_balance: f64) -> Result<Account, &'static str> {
        let account = bank.create_account(initial_balance)?;
        self.accounts.push(account.clone());
        println!("Opened new account for {} with balance ${}", self.name, initial_balance);
        Ok(account)
    }

    // Method to get a reference to a specific account by account number
    pub fn get_account(&self, account_number: &str) -> Option<&Account> {
        self.accounts.iter().find(|&account| account.get_account_number() == account_number)
    }

    // Method to get the customer's name
    pub fn get_name(&self) -> &str {
        &self.name
    }


    // Additional methods to manage customer information (e.g., update address, phone number) can be added here
}
