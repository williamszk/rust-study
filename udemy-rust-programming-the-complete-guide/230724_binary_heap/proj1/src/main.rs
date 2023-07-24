use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // there are two ways of creating maps/dicts in Rust
    // using HashMap and using a BinaryTree
    // both have the same methods

    let mut hmap = HashMap::new();

    hmap.insert(1, 1);
    hmap.insert(5, 2);
    hmap.insert(30, 3);
    let old_value = hmap.insert(30, 4);

    println!("{:?}", hmap);
    println!("old_value: {:?}", old_value);

    // check if a key exists
    println!("hmap.contains_key(&30): {}", hmap.contains_key(&30));
    println!("hmap.contains_key(&40): {}", hmap.contains_key(&40));

    // how to print the value give a certain key
    println!("hmap.get(&30): {:?}", hmap.get(&30));

    // we can also remove a certain key from the hash map
    println!("before: {:?}", hmap);
    let removed_value = hmap.remove(&30);
    println!("after: {:?}", hmap);
    println!("removed_value: {:?}", removed_value);

    // we can also use the method `remove_entry` which will return the key and the value
    let removed_key_value = hmap.remove_entry(&5);
    println!("removed_key_value: {:?}", removed_key_value);

    // we can also remove everything from inside the hashmap using:
    hmap.clear();
    println!("cleared hash map: {:?}", hmap);
    println!("hmap.is_empty(): {:?}", hmap.is_empty());
}
