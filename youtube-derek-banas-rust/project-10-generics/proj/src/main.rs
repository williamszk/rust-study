// here we are importing the Add trait
use std::ops::Add;

fn main() {
    println!("Hello, world!");
    println!("{}", get_sum_gen(10.2, 10.3)); // here we sum two floats
    println!("{}", get_sum_gen(10, 10)); // here we sum two integers
    // the get_sum_gen uses generics and use the Add trait to specify 
    // that the types that are accepted need to have the trait of Add
    // why would we need generics in this case instead of just passing the
    // trait as a type, like an interface in Go?
}

// traits

// fn get_sum_gen<T>(x: T, y: T)->T{
//     return x+y;
// }

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    // here we are including the Add trait into the signature of the function
    // are traits like interfaces in Go?
    return x + y;
}

// 50, 31 : 2G
//  142, 44 : 5G