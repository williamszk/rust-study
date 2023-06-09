fn main() {
    println!("Hello, world!");

    let course1 = Course {
        headline: String::from("This is the headline!"),
        author: String::from("Bob"),
    };

    let course2 = AnotherCourse {
        headline: String::from("This is the headline from another course!"),
        author: String::from("Alice"),
    };

    println!("{}", course1.overview());
    println!("{}", course2.overview());
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This is the default implementation")
    }
}

struct Course {
    headline: String,
    author: String,
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

impl Overview for AnotherCourse {
    // suppose AnotherCourse doesn't have an implementation for the overview method
    // what will happen?
    // fn overview(&self) -> String {
    //     format!("{}, {}", self.author, self.headline)
    // }
}
