#[allow(unused_variables)]

fn main() {
    let a: i16 = 12;
    let b: u32 = 4;
    let c: f32 = 3.7;

    // convert all values to i8
    println!("{}", a as i8 + b as i8 + c as i8);
    // c is a float but it is converted to i8

    // let a = 500 as i8; // those are going to give an error
    // let b = 100_000 as u16; // those are going to give an error
    // let c = 10_000_000_000 as u32; // those are going to give an error
    // println!("{} {} {}", a, b, c); // those are going to give an error

    // because the values there do not fit in the size of the type
    // i8's max is 127
    // u32's max is 4294967295 (2^32 - 1).

    let a: i16 = -150;
    let b = -150 as i16; // we can use type suffix
    let c = -150 + b - b;
    let d = -150_i16;
    let e = 100_000_000;
}
