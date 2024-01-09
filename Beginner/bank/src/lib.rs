pub struct SavingsAccount {
    balance: i32,
}

impl SavingsAccount {
    pub fn new() -> SavingsAccount {
        SavingsAccount { 
            balance: 0, 
        }
    }
    pub fn get_balance(&self) -> i32{self.balance}

    pub fn deposit(&mut self, ammount: i32) {
        if ammount < 0 {
            panic!("Can not deposit a negative amount!");
        }
        self.balance += ammount;
    }
    
    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }

}


#[cfg(test)]
mod unit_tests;