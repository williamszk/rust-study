use std::fs::File;

fn main() {
    println!("Hello, world!");
    _vec_example();
    _open_file();
    _if_let_syntax_open_file();
    let _ = _example_file_should_exist();
    let _ = _example_if_file_not_exist_create_it();
    // _example_unwrap_to_simplify_match()
}

fn _error_propagation() -> Result<std::fs::File, std::io::Error> {
    // propagate error out of the function
    let file_result = File::open("file_no_exist.txt")?;
    // the ? in the end will return the Error variant automatically
    // this is a short cut
    // we could do a match and return the Error manually
    // but the ? shortens our code
    // The Result has this special notation with `?`
    // 
    Ok(file_result)
}

fn _example_unwrap_to_simplify_match() {
    // We can use the `unwrap` method from Result if we want to just panic
    // if the result is not Ok(T)
    // `unwrap()` will just panic, without telling us any message
    // `expect()` will panic but we can pass a message in it so it is more customizable

    // this will panic
    let _file_result = File::open("file_no_exist.txt").unwrap();

    // this will panic with a custom error message
    let _file_result = File::open("file_no_exist.txt").expect("Panic!!! No file found");
}

fn _example_if_file_not_exist_create_it() -> File {
    let _file_result = File::open("file_no_exist.txt");
    let _file_result = File::open("my_file.txt"); // this works
    let file = match _file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("created_file.txt") {
                Ok(file_created) => file_created,
                Err(error) => panic!("Panic! {}", error),
            },
            _ => {
                panic!("Panic! I don't know what to do")
            }
        },
    };

    return file;
}

fn _example_file_should_exist() -> File {
    let file_result = File::open("my_file.txt"); // this works

    // let file_result = File::open("file_no_exist.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Panic!!! {}", error),
    };

    return file;
}

fn _if_let_syntax_open_file() {
    // using "if let" syntax;
    // this syntax is shorter than using the match
    let file = File::open("my_file.txt"); // this works

    // let file = File::open("file_no_exist.txt");

    if let Ok(content) = file {
        println!(
            "Banana; Success in opening the file; content: {:?}",
            content
        )
    }
}

fn _vec_example() {
    // panic!("Panic here"); // manual panic
    let vec1 = vec![1];

    // vec1[1]; // this will panic

    match vec1.get(1) {
        Some(val) => println!("{}", val),
        None => println!("Couldn't fine the value"),
    }
}

fn _open_file() {
    // let file = File::open("my_file.txt"); // this works
    let file = File::open("file_no_exist.txt");
    match file {
        Ok(content) => println!("Apple; Success in opening the file; content: {:?}", content),
        Err(error) => println!("Apple;Error! {}", error),
    }
}
