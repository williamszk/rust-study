fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    let result2;
    {
        result = longest(string1.as_str(), string2.as_str());
        result2 = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    println!("The longest string is {}", result2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}