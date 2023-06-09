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

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);

    call_overview_2(&course1);
    call_overview_2(&course2);

    call_overviews_01(&course1, &course2); // this works

    // call_overviews_02(&course1, &course2); // this doesn't work, because course1 and 2 don't have the same type
    call_overviews_02(&course1, &course1); // this works

    call_overviews_03(&course1, &course2); // this works
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

fn call_overview(item: &impl Overview) {
    println!("Overview: {}", item.overview());
}

// this is another way to use a trait as type for a parameter
fn call_overview_2<T: Overview>(item: &T) {
    println!("Overview: {}", item.overview());
}

// use just trait bounds
// in this case item1 and item2 can be two different concrete types
fn call_overviews_01(item1: &impl Overview, item2: &impl Overview) {
    println!("Overview item1: {}", item1.overview());
    println!("Overview item2: {}", item2.overview());
}

// in this case item1 and item2 need to be the same concrete type
fn call_overviews_02<T: Overview>(item1: &T, item2: &T) {
    println!("Overview item1: {}", item1.overview());
    println!("Overview item2: {}", item2.overview());
}

// to use generic notation to solve the problem we could use multiple generic variables
fn call_overviews_03<T: Overview, U: Overview>(item1: &T, item2: &U) {
    println!("Overview item1: {}", item1.overview());
    println!("Overview item2: {}", item2.overview());
}

// fn overview(item: &impl Overview + AnotherTrait);
// fn overview<T: Overview + AnotherTrait>(item: &T);
