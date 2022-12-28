fn main() {
    let s1 = String::from("This is a string in a bad situation");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}

fn calculate_length(some_str: String) -> (String, u32){
    let len: u32 = some_str.len() as u32;
    return (some_str, len)
}
