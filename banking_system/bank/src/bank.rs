use crate::customer::Customer;
use crate::account::Account;
use crate::admin::Admin;

pub struct Bank {
    name: String,
    location: String,
    customers: Vec<Customer>,
    accounts: Vec<Account>,
    admins: Vec<Admin>,
}

impl Bank {
    // Constructor to create a new Bank
    pub fn new(name: String, location: String) -> Self {
        Bank {
            name,
            location,
            customers: Vec::new(),
            accounts: Vec::new(),
            admins: Vec::new(),
        }
    }

    // Method to create a new account for a customer
    pub fn create_account(&mut self, initial_balance: f64) -> Result<Account, &'static str> {
        let account_number = format!("{}", self.accounts.len() + 1); // Generate a unique account number
        let mut account = Account::new(account_number, initial_balance);
        self.accounts.push(account.clone());
        Ok(account)
    }

    // Method to get a reference to a specific account by account number
    pub fn get_account(&self, account_number: &str) -> Option<&Account> {
        self.accounts.iter().find(|&account| account.get_account_number() == account_number)
    }

    // Method to add a customer to the bank
    pub fn add_customer(&mut self, customer: Customer) {
        self.customers.push(customer);
    }

    // Method to add an admin to the bank
    pub fn add_admin(&mut self, admin: Admin) {
        self.admins.push(admin);
    }



    // Additional methods to manage bank operations (e.g., view reports, manage admins) can be added here
}


