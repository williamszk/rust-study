use std::ops::Add;

fn main() {
    println!("Hello, world!");

    let coord = Point { x: 5.0, y: 7.0 };
    let coord2 = Point { x: 2.0, y: -1.0 };

    let sum = coord + coord2;

    // println!("sum.x: {}", sum.x);
    // println!("sum.y: {}", sum.y);

    println!("{:?}", sum);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
        }
    }
}
