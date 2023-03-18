#[macro_use]
extern crate rocket;

// fn main() {
//     println!("Hello, world!");
// }

#[get("/")]
fn index<'a>() -> &'a str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    // println!("Server running in port 8080.");
    rocket::build().mount("/", routes![index])
}
