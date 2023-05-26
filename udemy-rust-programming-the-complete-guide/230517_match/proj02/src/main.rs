fn main() {
    // https://www.udemy.com/course/rust-programming-the-complete-guide/learn/practice/1394854/introduction#overview
    println!("Hello, world!");

    let tri = Shape::Triangle;

    let a = tri.corners();
    println!("{}", a);

    let square = Shape::Square;
    println!("{}", square.corners());

    let pentagon = Shape::Pentagon;
    println!("{}", pentagon.corners());

}

// each of the variants of the enum below are unit structs
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(&self) -> i32 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}
