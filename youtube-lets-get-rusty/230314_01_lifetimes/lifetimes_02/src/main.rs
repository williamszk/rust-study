fn main() {
    let string1 = String::from("bananas");
    let string2 = String::from("apples");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

// &i32 a reference
// &'a i32 a reference with an explicit lifetime
// &'a mut i32 a mutable reference with and explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // use generic lifetime annotation
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
