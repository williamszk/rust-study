fn main() {
    let user1 = User {
        active: true,
        username: String::from("Bob"),
        sign_in_count: 0,
    };
    println!("{}", user1.username);

    let user2 = build_user(String::from("Alice"));
    println!("{}", user2.active);
    println!("{}", user2.username);
    println!("{}", user2.sign_in_count);


    let coords = Coordinates(1,2,3);

}

// Named field struct
// Tuple like struct
// Unit like struct

struct User {
    active: bool,
    username: String,
    sign_in_count: u32,
}

fn build_user(username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 1,
    }
}

// An example of tuple like structs
struct Coordinates(i32, i32, i32);

// Unit like struct
struct UnitLikeStruct;






