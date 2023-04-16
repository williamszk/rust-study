fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };

    println!("The result is {}", result);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The element is: {}", element);
    }


    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

}

