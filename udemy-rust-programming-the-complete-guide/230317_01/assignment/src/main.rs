fn main() {
    println!("Hello, world!");

    let mut car = Car{
        mpg: 10,
        color: String::from("Green"),
        top_speed: 80,
    };

    println!("{:?}", car);

    car.set_mpg(30);
    car.set_color(String::from("Blue"));
    car.set_top_speed(90);
    println!("{:?}", car);
}

#[derive(Debug)]
struct Car {
    mpg: i32,
    color: String,
    top_speed: i32,
}

impl Car {
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
