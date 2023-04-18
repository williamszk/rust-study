fn main() {
    println!("Hello, world!");

    let dog = Pet::Dog;

    println!("{}", dog.what_am_i());

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: "192.168.15.3",
    };

    println!("{:?}", home);

    let other_ip = IpAddr2::V4("192.168.15.3");

    println!("{:?}", other_ip);
}
enum Pet {
    Dog,
    Cat,
    Fish,
}
// all of the variants are unit structs
// the unit struct does not have fields
// they are mostly relevant inside Enum or used as a constant

// implement methods on Enums
impl Pet {
    fn what_am_i(&self) -> &str {
        match &self {
            Pet::Dog => "I am a Dog",
            Pet::Cat => "I am a Cat",
            Pet::Fish => "I am a Fish",
        }
    }
}
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr<'a> {
    kind: IpAddrKind,
    address: &'a str,
}

#[derive(Debug)]
enum IpAddr2<'a> {
    V4(&'a str),
    V6(&'a str),
}
