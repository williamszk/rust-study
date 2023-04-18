// https://www.udemy.com/course/rust-programming-the-complete-guide/learn/lecture/30464500#overview
fn main() {
    println!("Hello, world!");

    let some_number = Option::Some(5);

    println!("{:?}", some_number);

    let nothing: Option<i32> = Option::None;
    println!("{:?}", nothing);

    let x = 9;
    let y = Option::Some(10);

    // let z = x + y; // this won't work
    let z = x + y.unwrap();
    println!("This value came from an Option: {}", z);

    

}

// This is the implementation of enum
// enum Option<T>{
//     None,
//     Some(T)
// }
