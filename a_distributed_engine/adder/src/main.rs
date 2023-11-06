use add_one;

fn main() {
    println!("Hello, world!");

    let res = add_one::add_one(10);
    println!(
        "This is using the package add_one, and it's being called from add package result: '{}'",
        res
    );
}
