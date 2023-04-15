// iterating over values in a vector
fn main() {
    println!("Hello, world!");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    println!("----------------------------------------------------------");
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // variable i
    // address: 0xfff120
    // value: 0xfff123
    // type: pointer to the type

    // a value in memory
    // addr: 0xfff123
    // value: 10
    // type: int

    // *i (this is dereference)
    // *i -> 10
    // addr(i) ->  0xfff120
    // addr(*i) -> 0xfff123

    for i in &v {
        println!("{}", i);
    }
}
