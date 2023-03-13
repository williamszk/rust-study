fn main() {
    println!("Hello, world!");

    let my_square = Square {
        width: 5,
        height: 5,
    };

    println!("{}", my_square.area());
    println!("{}", my_square.width());

    let mut my_square = Square {
        width: 5,
        height: 5,
    };
    println!("Before width: {}", my_square.width());
    println!("Before area: {}", my_square.area());
    my_square.set_width(10);
    println!("After width: {}", my_square.width());
    println!("After area: {}", my_square.area());
}

struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}
