
fn main() {
    let a_str = String::from("Hello from fn");
    takes_ownership(a_str);

    // println!("second run: {}", a_str); // we can't use a_str here again

    let another_str = String::from("Hello from another str");
    borrows_from_value(&another_str);
    println!("{}", another_str); // this one will work
    // because the value was borrowed by the function, the function
    // did not took ownership
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn borrows_from_value(str_ref: &String){
    println!("{}", str_ref);
}