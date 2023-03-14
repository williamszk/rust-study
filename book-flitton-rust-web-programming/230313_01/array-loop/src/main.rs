fn main() {
    // Looping through ====================================================
    let int_array: [i32; 3] = [1, 2, 3];

    for i in int_array {
        println!("{}", i);
    }

    println!("{}", int_array[1]);

    // Example of mutable array ====================================================
    let mut mutable_array: [i32; 3] = [1, 2, 0];

    mutable_array[2] = 3;

    println!("{:?}", mutable_array);
    println!("{}", mutable_array.len());

    // Take slice of array ====================================================
    let slice_array: [i32; 100] = [0; 100];
    println!("length: {}", slice_array.len());
    println!("slice: {:?}", &slice_array[5..8]);

    // About Enums ====================================================
    enum SomeValue {
        StringValue(String),
        IntValue(i32),
    }

    let multi_array: [SomeValue; 4] = [
        SomeValue::StringValue(String::from("one")),
        SomeValue::IntValue(2),
        SomeValue::StringValue(String::from("three")),
        SomeValue::IntValue(4),
    ];

    for i in multi_array {
        match i {
            SomeValue::StringValue(data) => {
                println!("The string is: {}", data);
            }
            SomeValue::IntValue(data) => {
                println!("The int is: {}", data)
            }
        }
    }
    // About Vectors ====================================================
    let mut string_vector: Vec<&str> = vec!["one", "two", "three"];
    println!("{:?}", string_vector);
    string_vector.push("four");
    println!("{:?}", string_vector);

    // Create an empty vector
    let _empty_vector: Vec<&str> = Vec::new();

    // About HashMaps ====================================================
    #[derive(Debug)]
    enum CharacterValue {
        Name(String),
        Age(i32),
        Items(Vec<String>),
    }

    use std::collections::HashMap;

    let mut profile: HashMap<&str, CharacterValue> = HashMap::new();

    profile.insert("name", CharacterValue::Name("Maxwell".to_string()));
    profile.insert("age", CharacterValue::Age(32));
    profile.insert(
        "items",
        CharacterValue::Items(vec![
            "laptop".to_string(),
            "book".to_string(),
            "coat".to_string(),
        ]),
    );

    println!("{:?}", profile);

    let an_option = profile.get("name");

    match an_option {
        Some(data) => match data {
            CharacterValue::Name(name) => {
                println!("the name is: {}", name);
            }
            _ => {
                panic!("name should be a string.")
            }
        },
        None => {
            println!("name is not present!");
        }
    }


    let an_option = profile.get("name");
    match an_option.unwrap() {
        CharacterValue::Name(data) => {
            println!("the name is: {}", data);
        },
        _ => {panic!("name should be a string")}
    }

    // This will give an error:
    // match profile.get("power").unwrap() {
    //     CharacterValue::Name(data) => {
    //         println!("the name is: {}", data);
    //     },
    //     _ => {panic!("name should be a string")}
    // }


}
