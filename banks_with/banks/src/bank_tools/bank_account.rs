#[derive(Clone, Debug)]
pub struct BankAccount {
    pub id: u32,
    pub balance: u32,
}

impl BankAccount {
    /// Substract value from account balance
    pub fn substruct_from_balance(&mut self, value: u32) {
        println!("delete some value");
        self.balance -= value;
    }

    /// Add value to account balance
    pub fn add_to_balance(&mut self, value: u32) {
        println!("add some value");
        self.balance += value;
    }
}