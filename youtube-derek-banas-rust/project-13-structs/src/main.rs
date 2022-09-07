fn main() {
    // https://youtu.be/ygL_xcavzQ4?t=5094
    println!("Hello, world!");

    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Martin"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };

    // change Bob's address
    bob.address = String::from("505 Main St");

    println!("name: {}", bob.name);
    println!("address: {}", bob.address);
    println!("balance: {}", bob.balance);

    // we can use generics inside structs
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }

    let rectangle = Rectangle {
        length: 4,
        height: 10.3,
    };

    println!("length: {}", rectangle.length);
    println!("height: {}", rectangle.height);
}
