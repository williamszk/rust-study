fn main() {
    println!("Hello, world!");

    let five = Some(5);
    let result = plus_one(five);
    println!("result: {:?}", result);

    let just_none = None;
    let result = plus_one(just_none);
    println!("result: {:?}", result);

    what_pet("Dog");
    what_pet("Fish");
    what_pet("Cat");

    // https://www.udemy.com/course/rust-programming-the-complete-guide/learn/lecture/30465160#overview
    // if-let
    let dog2 = Some(Pet::Dog);
    let cat2 = Some(Pet::Cat);
    let fish2 = Some(Pet::Fish);

    if let Some(Pet::Dog) = dog2 {
        println!("The animal is a dog!");
    } else {
        println!("Not a dog!");
    }

    // another example of while-let statement with pop and a vector
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // top is defined in the while let statement
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // we can use vectors as stacks

    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        _ => println!("The rest"),
    }

    match x {
        1..=5 => println!("Matches"),
        _ => println!("Not matching"),
    }

    let x = Some(5);
    let y = 5;

    match x {
        Some(10) => println!("10!"),
        Some(value) if value == y => println!("Matches!"), // this is new
        _ => println!("Default!"),
    }
}

enum Pet {
    Dog,
    Cat,
    Fish,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn what_pet(input: &str) {
    // match can work like a switch
    match input {
        "Dog" => println!("I have a Dog!"),
        "Fish" => println!("I have a Fish!"),
        _ => println!("I don't know what this is."),
    }
}
