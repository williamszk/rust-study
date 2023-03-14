// Chapter 10 - Defining Generic Functions and Structs
fn main() {

    fn _my_func_01(ch: char, num1: i16, num2: i16)->i16 {
        if ch == 'a' {num1}
        else {num2}
    }

    println!("{}", _my_func_01('a', 37, 42));

    // Solve the problem of having to support many types of numbers by using generic 
    fn _my_func_02<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' {num1}
        else {num2}
    }

    println!("------- Using generic function -------");
    println!("{}", _my_func_02::<i32>('a', 37, 42));
    println!("{}", _my_func_02::<f32>('a', 37.2f32, 42.3f32));

    // Stopped at:
    // Inferring the Parametric Types
    // p. 124

}
