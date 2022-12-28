fn main() {
    let mut my_str = String::from("A string from main... ");
    println!("Before: {}", my_str);
    change(&mut my_str); // This function will change the value of my_str, implicitly
    println!("After: {}", my_str);
}


fn change(str: &mut String){
    str.push_str("I was changed!");
}