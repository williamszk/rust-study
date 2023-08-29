pub fn _main() {
    println!("Hello, world!");
    // _experiment_01();
    // _iterating_over_vector();
    // _attempting_to_mutate_while_holding_a_reference();
    _iterating_and_changing_vector();
}

fn _iterating_and_changing_vector() {
    let mut v = vec![100, 32, 57];
    println!("_iterating_and_changing_vector - original: {:?}", v);
    for i in &mut v {
        *i += 50;
        // dereferencing and adding 50 to the element of the vector
    }
    println!("_iterating_and_changing_vector -  changed: {:?}", v);
}

fn _experiment_01() {
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3];

    let mut _v = Vec::new();
    _v.push(1);
    _v.push(2);
    _v.push(3);
    _v.push(4);
    _v.push(5);

    let third = &_v[2]; // this notation will panic if the index does not exist
    println!("The third element of _v is: '{}'.", third);
    // let _will_panic = &_v[10];

    let index = 2; // <-- change this value to 10 or 2 and see the result
    match _v.get(index) {
        Some(num) => println!("The value in index '{}' is '{}'.", index, num),
        None => println!("There is no value in index '{}'.", index),
    }
}

fn _attempting_to_mutate_while_holding_a_reference() {
    let mut v = vec![1, 2, 3, 4, 5];
    let _first = &v[0];
    v.push(6);
    // println!("The first element is: {}", _first); // this will break
}

fn _iterating_over_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
