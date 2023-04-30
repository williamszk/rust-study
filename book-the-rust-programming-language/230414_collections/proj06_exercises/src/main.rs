use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // ========================================================================
    // Given a list of integers, use a vector and return the mean (the average
    // value), median (when sorted, the value in the middle position), and
    // mode (the value that occurs most often; a hash map will be helpful
    // here) of the list.

    let my_vec = vec![1, 2, 3, 8, 4, 6, 5, 8, 9, 10, 2, 3, 3];
    let mut sum: i32 = 0;
    // get the average value of the vector ====================================
    for i in &my_vec {
        sum += *i;
        // sum += i; // why I can do this?
    }
    println!("my_vec: {:?}", my_vec);
    // println!("sum: {}", sum);
    let length = my_vec.len();
    // println!("length: {}", length);
    let result = sum as f64 / length as f64;
    println!("average: {}", result);
    // get the median =========================================================
    let mut my_vec_2 = my_vec.clone();
    my_vec_2.sort();
    // println!("my_vec_2: {:?}", my_vec_2);
    let length = my_vec_2.len();
    let numerator: f64;
    let denominator: f64;
    if length % 2 == 0 {
        let left = length / 2 - 1;
        let right = left + 1;
        // println!("left: {}", left);
        // println!("right: {}", right);
        numerator = (my_vec_2[left] + my_vec_2[right]) as f64;
        denominator = 2.0;
    } else {
        let index = length / 2;
        // println!("index: {}", index);
        numerator = my_vec_2[index] as f64;
        denominator = 1.0;
    }
    let median = numerator / denominator;
    println!("median: {}", median);

    // get the mode =========================================================
    // calculate the frequency using a hash map
    let mut frequency = HashMap::new();
    for num in &my_vec {
        let count = frequency.entry(num).or_insert(0);
        *count += 1;
    }
    // println!("{:?}", frequency);

    // iterate over the hashmap and keep track of the largest value and the
    // associated key
    let mut max_val = 0;
    let mut max_key = 0;

    for (key, val) in &frequency {
        if *val > max_val {
            max_key = **key;
            max_val = *val;
        }
    }
    println!("mode: {:}", max_key);
    // is there a more high level way to calculate the mode from a hashmap?
    // or even a vector?

    // there is a more functional way to do this calculation
    let mode = frequency
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&num, _)| num)
        .unwrap();
    println!("mode: {:}", mode);

    // Still is pretty hard to understand completely what is going on on this code.
    // how those two ways of calculating the mode perform?
    // ========================================================================
    // Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “first-fay.”
    // Words that start with a vowel have “hay” added to the end instead
    // (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8
    // encoding!
}

// ========================================================================
// Using a hash map and vectors, create a text interface to allow a user to
// add employee names to a department in a company. For example, “Add
// Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve
// a list of all people in a department or all people in the company by
// department, sorted alphabetically.

// ========================================================================
