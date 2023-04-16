fn main() {
    println!("Hello, world!");
    // the two primitive compound types are: tuples and arrays...
    // all the rest of compound types are based on those two

    let tup: (i32, f64, u8) = (500i32, 6.4f64, 1u8);
    println!("{:?}", tup);

    // destructuring tuple values
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    // access values in tuple:
    println!("x.0={}", tup.0);
    println!("x.1={}", tup.1);
    println!("x.2={}", tup.2);

    // arrays:
    let arr = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // Using arrays to represent months is a good idea, because it is unlikely
    // that this array will change in size...

    let arr:[i32; 5] = [1,2,3,4,5];

    let arr = [0;10]; // an array of 0s with 10 elements, note the semicolon
    println!("{:?}", arr); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]



}
