use crate::{bank::Bank, account::Account};
pub struct Admin {
    username: String,
    password: String,
}

impl Admin{
    pub fn new(username: String, password: String) -> Self {
        Admin {
            username,
            password,
        }
    }
    pub fn manage_accounts(&self, bank: &mut Bank) {
        println!("Admin {} managing accounts", self.username);

    }
    pub fn manage_reports(&self, bank: &mut Bank) {
        println!("Admin {} managing reports", self.username);

    }
    

        
}