fn main() {
    println!("Hello, world!");
}


// https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html#:~:text=clearer%20to%20you.-,Returning%20Closures,return%20value%20of%20the%20function.
// This seems hard for me right now... I'll wait till understand more about box
fn back_account(amount_saved: f64) -> fn(amount_spend:f64)->f64 {
    let spend_money = |amount_spent: f64| -> f64 {
        amount_saved -= amount_spent;
        amount_saved
    };

    spend_money
}
