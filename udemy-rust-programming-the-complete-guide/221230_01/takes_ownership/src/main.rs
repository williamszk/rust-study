fn main() {

    let x = String::from("David Rust");

    takes_ownership(x); // <- this function will have the ownership of the object now
    // println!("{}", x); // <- we can't use x anymore

    let x = String::from("Pelé Messi");
    doesnt_take_ownership(&x); // <- this function will not take ownership it will just use a immutable reference
    println!("Monkey 01: {}", x); // <- this works because x still has the ownership of the object


    // Some call the "immutable reference" also a "shared reference".
    // Which makes sense, because we can have many immutable references.

}

fn takes_ownership(str: String){
    println!("{}", str);
}

fn doesnt_take_ownership(str: &String){
    println!("{}", str);
}