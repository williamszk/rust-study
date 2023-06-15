use std::fmt::Debug;

fn main() {
    println!("Hello, world!");

    let mut car = Car {
        mpg: 10,
        color: String::from("Green"),
        top_speed: 80,
    };

    my_special_print(&car);

    car.set_mpg(30);
    car.set_color(String::from("Blue"));
    car.set_top_speed(90);

    my_special_print(&car);

    // ==============================================================

    let mut motorcycle = Motorcycle {
        mpg: 10,
        color: String::from("Green"),
        top_speed: 80,
    };

    my_special_print(&motorcycle);

    motorcycle.set_mpg(100);
    motorcycle.set_color(String::from("Red"));
    motorcycle.set_top_speed(1000);

    my_special_print(&motorcycle);
}

trait Vehicle {
    fn set_mpg(&mut self, new_mpg: i32);
    fn set_color(&mut self, new_color: String);
    fn set_top_speed(&mut self, new_top_speed: i32);
}

#[derive(Debug)]
struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Vehicle for Car {
    fn set_mpg(&mut self, new_mpg: i32) {
        self.mpg = new_mpg
    }
    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }
    fn set_top_speed(&mut self, new_top_speed: i32) {
        self.top_speed = new_top_speed
    }
}

#[derive(Debug)]
struct Motorcycle {
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Vehicle for Motorcycle {
    fn set_mpg(&mut self, new_mpg: i32) {
        self.mpg = new_mpg
    }
    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }
    fn set_top_speed(&mut self, new_top_speed: i32) {
        self.top_speed = new_top_speed
    }
}

fn my_special_print<T: Debug>(object: &T) {
    println!("{:?}", object);
}
