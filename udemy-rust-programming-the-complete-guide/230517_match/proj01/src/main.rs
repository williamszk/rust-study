fn main() {
    println!("Hello, world!");

    let five = Some(5);
    let result = plus_one(five);
    println!("result: {:?}", result);

    let just_none = None;
    let result = plus_one(just_none);
    println!("result: {:?}", result);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
