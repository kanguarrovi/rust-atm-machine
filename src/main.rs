#[warn(unused_must_use)]

pub mod bank;
use bank::Bank;

use std::io;

fn main() {
    let mut _bank = Bank::default();

    let mut choice;

    loop {

        println!("-------------------WELCOME-------------------");
        println!("Enter your choice: ");
        println!("1. Enter name. Account number. Account type");
        println!("2. Balance Enquiry");
        println!("3. Deposit money");
        println!("4. Show total balance");
        println!("5. Withdraw money");
        println!("6. Cancel\n");


        let mut string_choice = String::new();
        io::stdin().read_line(&mut string_choice).expect("Failed to read line");
        choice = string_choice.trim().parse().expect("Failed to parse");

        match choice {
            1 => {
                _bank.set_value();
            },
            2 => {
                _bank.show_data();
            },
            3 => {
                _bank.deposit();
            },
            4 => {
                _bank.show_balance();
            },
            5 => {
                _bank.withdraw();
            },
            6 => {
                break;
            },
            _ => {
                println!("Invalid choice!");
            }
        };

        println!("---------------------------------------------\n");

    }
}