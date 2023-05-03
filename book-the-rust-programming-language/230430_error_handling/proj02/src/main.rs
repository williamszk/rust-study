// This program will do the same as in proj01
// but here we use the unwrap_or_else method of Result
// instead of dealing with match
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
