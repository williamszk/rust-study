fn main() {
    println!("Hello, world!");

    let coord = Point { x: 5.0, y: 9.0 };
}

struct Point<T> {
    x: T,
    y: T,
}
