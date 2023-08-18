fn main() {
    println!("Hello, world!");
    let mut x: i32;
    // assert_eq!(x, 42);
    x = 42;

    let y = &x;

    // let x = 43;

    // with this, the program crashes
    x = 43;

    assert_eq!(*y,42);
}
