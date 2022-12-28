fn main() {
    let s1 = String::from("This is a string in a bad situation");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

// Passing references to functions is called borrowing.
// The function is just borrowing the object.
// Underneath the reference is pointing to the variable which in turn points
// to the object.
// When the reference is deleted, the variable is not deleted.
// References are immutable by default.
fn calculate_length(some_str: &String) ->  u32 {
    let len: u32 = some_str.len() as u32;
    return len
}


