
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("bob@martin.com"),
        username: String::from("Bob"),
        active: true,
        sign_in_count: 1,
    };

    //
    let name = user1.username;
    println!("{}", name);

    // change the field of struct
    user1.username = String::from("Dennis");

    let user2 = build_user(
        String::from("ken@bell.com"), 
        String::from("Ken"));

    println!("{:?}", user2);

    // we can reuse the fields of another struct
    // when creating the new struct
    let user3 = User {
        email: String::from("Derek@Banas.com"),
        username: String::from("Derek Banas"),
        ..user2
    };

    println!("{:?}", user3);

    let my_color = Color(0,1,255);
    println!("my_color: {:?}", my_color);

    let my_point = Point(0,1,-21);
    println!("my_point: {:?}", my_point);

    // using tuples
    let rect = (30, 50);
    println!("The area result: {}", area(rect));

    let rect2 = Rectangle{
        width: 14,
        height: 20,
    };

    println!("The area_struct {}", area_struct(&rect2));

    println!("The area method {}", rect2.area());

    let other_rect = Rectangle{
        width: 8,
        height: 19,
    };

    println!("Result of can hold: {}", rect2.can_hold(&other_rect));

    // calling the associated function
    let my_square = Rectangle::square(10);
    println!("printing the square from the associated function {:?}", my_square);

}

// method syntax
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

// what is the difference between methods and associated functions?
// associated functions are static methods
// they are methods that does not need an instance of the class
// to be ran
// we can just use the class/struct to run it
// in associated functions we do not pass the &self
// as the first argument

// we can build many implementation blocks
impl Rectangle {
    fn square(size:u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}




// we pass the argument as a reference because we want to use
// the fields of the argument, but not take ownsership
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(dimensions: (u32, u32))->u32{
    dimensions.0 * dimensions.1
}

// ==========================
// we can create structs without named fields
// those are called tuple structs
// tuple structs are useful when we need to give a name
// to a tuple, and bring meaning for the code

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    // we are using here the "field init" shorthand syntax
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
    }
}
