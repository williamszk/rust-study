fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello there I was born inside gives_ownership");
    some_string
}
