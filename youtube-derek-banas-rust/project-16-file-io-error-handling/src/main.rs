use std::fs::File;
/// This example program will create a file, write to the file
/// and then read from the file.
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let path = "lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();

    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // Handling different types of errors:
    let output2 = File::open("rand.txt");

    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => {
                    println!("I ran here: We are creating the not found file.");
                    fc
                }
                Err(error) => panic!("Can't create file {:?}", error),
            },
            _other_error => panic!("Problem opening file: {:?}", error),
        },
    };
}
// https://youtu.be/ygL_xcavzQ4?t=6349

// Next:
// https://youtu.be/ygL_xcavzQ4?t=6945
