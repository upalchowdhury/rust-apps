#![allow(unused)]
use std::sync::{Arc,Mutex};
use std::thread;

// Build a multithreaded and then an async application for banking


pub struct Bank {
    balance : f32,
}

fn withdraw(bank:Arc<Mutex<Bank>>,amt:f32) {
    let mut bank_ref = bank.lock().unwrap();
    if amt < 2.00 {
        println!("not enough balance")
    } else {
        bank_ref.balance -= amt;
        println!("Withdrawl maded {} Customer balance is {}", amt, bank_ref.balance);
    }
    
    
}

fn customer (bank: Arc<Mutex<Bank>>) {
    withdraw(bank,5.00);
}

 
fn handlers (bank:Arc<Mutex<Bank>>) {
    let handlers = (0..10).map(|_| {
    let bankref = bank.clone();
        
        thread::spawn(||{
            customer (bankref);
        })
    });
    
    
    for h in handlers {
        h.join().unwrap();
    }
    
    println!("The current balance is {}", bank.lock().unwrap().balance);
}






fn main() {
    
    let new_bank = Arc::new(Mutex::new(Bank{balance:300.00}));
    handlers(new_bank);
    
}
