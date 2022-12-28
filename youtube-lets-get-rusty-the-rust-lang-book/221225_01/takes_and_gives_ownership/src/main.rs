fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello, I was born in main");
    let s3 = takes_and_gives_ownership(s2);
    println!("From main: s3: {}", s3);
    println!("From main: s1: {}", s1);
}

fn gives_ownership()->String{
    let some_string = String::from("I'm a String, born inside gives_ownership function");
    some_string
}

fn takes_and_gives_ownership(input_string: String) -> String {
    println!("From takes_and_gives_ownership: {}", input_string);
    input_string
}