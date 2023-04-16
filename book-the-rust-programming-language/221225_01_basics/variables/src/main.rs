fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing:
    let x = x + 1;

    let spaces = "     ";
    let spaces = spaces.len();

}
