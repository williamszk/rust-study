fn main() {
    // https://youtu.be/ygL_xcavzQ4?t=5275
    println!("Hello, world!");
    const PI: f32 = 3.141592;

    // traits are like interfaces in OOP
    // we define behavior with traits
    // a set of functions that a certain struct needs to abide
    // so that we can say that the struct X has trait Y
    trait Shape {
        // the "new" function returns the Shape trait
        fn new(length: f32, width:f32) -> Self;
        fn area(&self) -> f32;
    }

    // In Go we would say that a certain struct implements an interface

    struct Rectangle {length:f32, width: f32}
    struct Circle {length:f32, width: f32}

    // how to implement a trait for a struct
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }


    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }

        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0) * PI;
        }
    }

    // we can say that A Rectable is a Shape
    // because Rectable nows implements the Shape trait
    let rec: Rectangle = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rectable area: {}", rec.area());
    println!("Circle area: {}", circ.area());

}
