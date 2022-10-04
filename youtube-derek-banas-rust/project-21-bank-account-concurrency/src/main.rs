// https://youtu.be/ygL_xcavzQ4?t=8459

use std::thread;

fn main() {
    println!("Hello, world!");

    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }

    let mut bank = Bank{
        balance: 100.0
    };

    withdraw(&mut bank, 5.00);

    println!("Balance: {}", bank.balance);


    fn customer(the_bank: &mut Bank){
        withdraw(the_bank, 5.00);
    }

    // https://youtu.be/ygL_xcavzQ4?t=8745
    thread::spawn(||{
        customer(&mut bank)
    }).join().unwrap();

}
