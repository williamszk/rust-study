use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_yaml;
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
    print_yaml_struct_unknown("./src/experiments/230708_01/empty_file.yaml");

    Ok(())
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
