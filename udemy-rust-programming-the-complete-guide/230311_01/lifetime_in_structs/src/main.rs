fn main() {
    println!("Hello, world!");

    let str1 = String::from("This is my string.");

    let x = MyString{
        text: str1.as_str()
    };
}

struct MyString <'a>{
    text: &'a str,
}
