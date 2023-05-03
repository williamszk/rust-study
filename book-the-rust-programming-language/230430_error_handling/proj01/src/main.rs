use std::fs::File;
use std::io::{Error, ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    // Result<File, std::io::Error>
    // T: File
    // E: Error
    // File: is a struct
    // Error: is a struct

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // "Result" is an "enum"
    // the variants of the Result enum are: "Ok" and "Err"
    // specifically this "Result" will have variants of type:
    // "File" and "Error" both of those types are structs
    //
}
