fn main() {
    let mut s1 = String::from("This is a string in a bad situation");
    let len = calculate_length(&mut s1); // This will alter s1...
    println!("The length of '{}' is {}", s1, len);
}

// The parameter some_str is of type "reference to String", I imagine that is
// like a pointer to a String.
// Reference types are immutable by default.
// We can make them mutable though. There are some restrictions when we do this.

// "&mut" means a mutable reference.
fn calculate_length(some_str: &mut String) ->  u32 {
    some_str.push_str(", oops...");
    let len: u32 = some_str.len() as u32;
    return len
}
