use std::collections::HashSet;

fn main() {
    // about sets
    println!("Hello, world!");

    let mut hset = HashSet::new();

    // get how many elements are there in the set
    hset.len();
    hset.is_empty();

    hset.insert(1);
    hset.insert(3);
    hset.insert(5);
    hset.insert(4);

    println!("{:?}", hset);

    for x in hset.iter() {
        println!("{}", x);
    }

    hset.remove(&3);
    println!("after removing: {:?}", hset);

    let mut hset2 = HashSet::new();

    hset2.insert(1);
    hset2.insert(3);
    hset2.insert(5);
    hset2.insert(7);

    // check intersection
    println!("intersection");
    for x in hset.intersection(&hset2) {
        println!("{}", x);
    }

    // another way of finding the intersection
    // a short hand way
    // we just use the `&` (binary bit wise) operator
    let intersection = &hset & &hset2;
    println!("intersection: {:?}", intersection);

    // Union of sets, this is just the short-hand way
    let union = &hset | &hset2;
    println!("union: {:?}", union);

    // find the difference
    // use the short had way
    let difference = &hset - &hset2;
    println!("difference: {:?}", difference);
}
