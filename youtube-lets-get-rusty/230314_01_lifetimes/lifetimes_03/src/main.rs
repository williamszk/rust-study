fn main() {
    let string1 = String::from("bananas");
    let result;
    {
        let string2 = String::from("apples");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // use generic lifetime annotation
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
