fn main() {
    let _var = 1; // created on the stack
    let mut s = "Hello".to_string(); // created on the heap
    s.push_str(", World!");
    println!("{}", s);

    // Move the ownership
    let x = vec!["Taylor".to_string()]; // Vectors are created on the heap
    let _y = x;
    // println!("{:?}", x); // <-- this will give an error, because x does not own any object anymore at this point

    let x = vec!["Swift".to_string()]; // Vectors are created on the heap
    let _y = x.clone(); // <- the clone method performs a deep copy
    println!("{:?}", x); // <- this will not give an error because we cloned the object of x into _y

    let x = vec!["Rihanna".to_string()];
    // make a "borrow reference"
    let _y = &x;
    println!("Example of borrow reference: {:?}", x);

    // To clone only makes sense for compound types, not scalar types
    // because scalar types already clone by default.
    let x = 10;
    let _y = x;
    println!("{}", x); // It works... we don't need to clone an i32.

    // The default copy/clone behavior happens with types that stored in the stack.
    // That is for types that are stored on the heap, the copy will not be the 
    // default behavior.

}
