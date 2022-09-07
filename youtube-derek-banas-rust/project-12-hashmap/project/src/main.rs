// start at:
// https://youtu.be/ygL_xcavzQ4?t=4809

use std::collections::HashMap;

fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    for (k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length: {}", heroes.len());

    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(_x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}
