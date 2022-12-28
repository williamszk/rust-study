fn main() {
    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); // this will break

    let x = 10;
    this_will_make_copy(x);
    println!("{}", x);
    // x is a scalar type, it will automatically copy/clone instead of pass
    // ownership to the function that uses it
}


fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn this_will_make_copy(some_int: i32){
    println!("{}", some_int);
}