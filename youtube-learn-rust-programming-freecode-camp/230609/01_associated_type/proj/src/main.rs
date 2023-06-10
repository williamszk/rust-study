use core::panic;

fn main() {
    println!("Hello, world!");

    let duck = Duck;
    duck.swim();
}

// -----------------------------------------------------------------------------
trait MyTrait {
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
    // note that MyType is being returned in the get_my_type function.
}

struct MyStruct {}

impl MyTrait for MyStruct {
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
        return 42;
    }
}

// -----------------------------------------------------------------------------

trait Bird {
    fn quack(&self) -> String;
}

// this is a unit struct
struct Duck;

impl Duck {
    fn swim(&self) {
        println!("Look! The duck is swimming");
    }
}

struct Swan;

impl Swan {
    fn fly(&self) {
        println!("Look! the duck... sorry, the swan is flying.")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

// 1 swan
// 2 duck
fn hatch_a_bird(species: i32) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}

// https://youtu.be/BpPEoZW5IiY?t=26167
