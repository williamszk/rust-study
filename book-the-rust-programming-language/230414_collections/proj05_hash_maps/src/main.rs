use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // ========================================================================
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    // ========================================================================
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{}", field_name);
    // ========================================================================

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(val) => println!("val 01: {}", val),
        None => println!("None"),
    }

    let score = scores.get(&"Blue".to_string());
    match score {
        Some(val) => println!("val 02: {}", val),
        None => println!("None"),
    }

    let score = scores.get(&"Red".to_string());
    match score {
        Some(val) => println!("val 02: {}", val),
        None => println!("None"),
    }
    // ========================================================================

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // ========================================================================
    // insert will replace the value inside the hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // ========================================================================
    // entry and or_insert will check if there are any values in
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let value = scores.entry(String::from("Yellow")).or_insert(50);
    println!("value1: {:?}", value);

    let value = scores.entry(String::from("Blue")).or_insert(50);
    println!("value2: {:?}", value);

    // ========================================================================
    // count frequency of words
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // ========================================================================
    //     Given a list of integers, use a vector and return the mean (the average
    // value), median (when sorted, the value in the middle position), and
    // mode (the value that occurs most often; a hash map will be helpful
    // here) of the list.

    // ========================================================================
    //     Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
    // encoding!

    // ========================================================================
    // Using a hash map and vectors, create a text interface to allow a user to
    // add employee names to a department in a company. For example, “Add
    // Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
    // a list of all people in a department or all people in the company by
    // department, sorted alphabetically.

    // ========================================================================

}
