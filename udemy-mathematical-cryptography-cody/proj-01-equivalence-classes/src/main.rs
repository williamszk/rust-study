
// build a function that finds equivalence classes

fn main() {

    // should we have a unsigned integer of size 128 or 256? 
    // maybe we should try to implement arithmetic operations
    let modulo: i64 = 12;

    // if module is m
    // then we have m - 1 equivalence classes
    // which are represented by: [0], [1], [2], [3], ... , [m-1]
    // an integer "a" will fall in one of those classes depending on
    // the residue of the modulo operation

    // show a table of the equivalence classes
    // range between -50 to 50
    let mut idx = -20;
    // this is one way to build loops in rust, there are others

    println!("(modulo {})", modulo);
    println!("{} \t {}", format!("{: >6}", "Number"), format!("{: >10}", "equi class"));
    loop {

        let equivalence_class = (idx % modulo).abs();

        println!("{} \t {}", format!("{: >6}", idx), format!("{: >10}", format!("[{}]", equivalence_class)));
        // how to pad to the right
        // https://stackoverflow.com/a/50458236/8782077


        if idx == 20 {
            break;
        } 
        idx += 1;
    }








}
