#![allow(unused)]

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_yaml;
use serde_yaml::Value;
use std::collections::HashMap;
use std::error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    // ------------------------------------------------------------------------
    // print_yaml_struct_known::<MyData>("./src/foo2.yaml");
    // ------------------------------------------------------------------------
    // print_yaml_struct_unknown("./src/foo3.yaml");
    // print_yaml_struct_unknown("./src/experiments/230708_01/experiment_with_table.yaml");
    // print_yaml_struct_unknown("./src/experiments/230708_01/empty_file.yaml");
    // print_yaml_struct_unknown("./src/experiments/230711_01/test01.yaml");

    let yaml_content = read_yaml("./src/experiments/230711_01/test01.yaml");

    // I should create a map in rust to handle the ids that I already used
    let mut all_ids: HashMap<String, Box<dyn Fn() -> String>> = HashMap::new();
    // The value of the HashMap is a function which does not receive anything and returns a string.

    if let Value::Mapping(mapping) = yaml_content {
        for key in mapping.keys() {
            println!("{:?}", key);
            let value = mapping.get(key).expect("Couldn't find the key!");
            println!("{:?}", value);
        }
    }

    Ok(())
}

fn print_keys_from_mapping(mapping: serde_yaml::Mapping) {
    for key in mapping.keys() {
        println!("{:?}", key);
    }
}

fn read_yaml(file_path: &str) -> Value {
    // An example using serde_yaml NOT knowing the structure in advance
    let file = File::open(file_path).expect("Failed to open the file.");

    // Parse the YAML file into a serde_yaml::Value object
    let yaml_content: Value = serde_yaml::from_reader(file).expect("Failed to parse YAML.");

    return yaml_content;
}

fn print_yaml_struct_unknown(file_path: &str) {
    // An example using serde_yaml NOT knowing the structure in advance
    let file = File::open(file_path).expect("Failed to open the file.");

    // Parse the YAML file into a serde_yaml::Value object
    let yaml_content: serde_yaml::Value =
        serde_yaml::from_reader(file).expect("Failed to parse YAML.");

    // Now you can work with the parsed YAML data
    // The structure can be inspected and accessed dynamically using methods provided by serde_yaml::Value

    // Example: Print the parsed YAML
    println!("{:#?}", yaml_content);
}

fn print_yaml_struct_known<T: DeserializeOwned + Debug>(file_path: &str) {
    // Open the YAML file
    let mut file = File::open(file_path).expect("Failed to open the file.");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file.");

    // Parse the yaml into your custom data structure
    let data: T = serde_yaml::from_str(&contents).expect("Failed to parse YAML.");

    println!("From foo2: {:?}", data);
}

#[derive(Debug, Deserialize)]
struct MyData {
    // Define the structure of your data
    // based on the YAML file's structure
    key1: String,
    key2: u32,
}
