fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true!");
    } else {
        println!("condition was false!");
    }

    // using let as an expression
    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is {}", number)


}
