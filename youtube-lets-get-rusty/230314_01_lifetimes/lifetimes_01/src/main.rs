fn main() {

    let r;

    {
        let x = 5;
        r = &x;
    } // the &x is removed here, and r is referencing something that does not exist

    // this will produce a dangling reference
    println!("r: {}", r);

}
