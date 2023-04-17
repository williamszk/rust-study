fn main() {
    println!("Hello, world!");
    // 3 types of structs

    // Name Field Struct
    // Tuple Like Struct
    // Unit Like Struct

    let user1 = User {
        active: true,
        username: String::from("Bob"),
        sign_in_count: 0,
    };

    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.sign_in_count);

    let user2 = build_user(String::from("Alice"));
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.sign_in_count);


    let coords = Coordinates(1,2,3);
    println!("{}", coords.0);
    println!("{}", coords.1);
    println!("{}", coords.2);

}

// This is an example of a named field struct
struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

// This is like a constructor of a struct
fn build_user(username: String) -> User {
    User {
        username,
        active: true,
        sign_in_count: 0,
    }
}

// This is an example of a Tuple Like Struct
struct Coordinates(i32, i32, i32);

struct UnitStruct;





