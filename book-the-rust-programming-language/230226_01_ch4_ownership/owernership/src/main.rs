fn main() {
    println!("Hello, world!");

    let _my_str = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // append a string literal into a String
    // note that String type is stored on the heap, because it can change size

    println!("{}", s);

}
