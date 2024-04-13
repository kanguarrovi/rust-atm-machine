use std::io;

pub struct Bank{
    name: String,
    acc_number: String,
    acc_type: String,
    amount: f32,
    tot:f32
}

impl Default for Bank {
    fn default() -> Bank {
       Bank {
            name: String::from(""),
            acc_number: String::from(""),
            acc_type: String::from(""),
            amount: 0.0,
            tot: 0.0
        }
    }
}

impl Bank{

    pub fn set_value(&mut self){
        println!("Enter your name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        self.name = name.trim().to_string();

        println!("Enter Account number:");
        let mut acc_number =String::new();
        io::stdin().read_line(&mut acc_number).expect("Failed to read line");
        self.acc_number = acc_number.trim().to_string();

        println!("Enter account type:");
        let mut acc_type = String::new();
        io::stdin().read_line(&mut acc_type).expect("Failed to read line");
        self.acc_type = acc_type.trim().to_string();

        println!("Enter Balance:");
        let mut balance =String::new();
        io::stdin().read_line(&mut balance).expect("Failed to read line");
        self.tot = balance.trim().parse().expect("Failed to parse");

    }

    pub fn show_data(&self){
        println!("Name: {}", self.name);
        println!("Account number: {}", self.acc_number);
        println!("Account type: {}", self.acc_type);
        println!("Balance: {}", self.tot);
    }

    pub fn deposit(&mut self){
        println!("Enter amount to be deposited");
        let mut amount =String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read line");
        self.amount = amount.trim().parse().expect("Failed to parse");
    }

    pub fn show_balance(&mut self){
        self.tot = self.tot + self.amount;
        println!("Total balance is: {}", self.tot);
    }

    pub fn withdraw(&mut self){
        println!("Enter amount to withdraw");
        let mut withdraw_amount =String::new();
        io::stdin().read_line(&mut withdraw_amount).expect("Failed to read line");
        let number_withdraw_amount: f32 = withdraw_amount.trim().parse().expect("Failed to parse");
        let aval_balance: f32 = self.tot - number_withdraw_amount;
        println!("The available balance is {}", aval_balance);
    }

}