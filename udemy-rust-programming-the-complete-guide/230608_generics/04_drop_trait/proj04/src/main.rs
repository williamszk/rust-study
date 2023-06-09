fn main() {
    println!("Hello, world!");

    let course1 = Course {
        headline: String::from("This is the headline!"),
        author: String::from("Bob"),
    };

    drop(course1);
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        println!("Dropping: {}", self.author);
    }
}
