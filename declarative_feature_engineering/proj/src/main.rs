use serde::Deserialize;
use serde_yaml;
use std::error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    // ------------------------------------------------------------------------
    // https://stackoverflow.com/questions/53243795/how-do-you-read-a-yaml-file-in-rust

    let file = File::open("./src/foo.yaml")?;
    let data: String = serde_yaml::from_reader(file)?;
    println!("Read YAML string: {:?}", data);
    // ------------------------------------------------------------------------
    // An example reading a yaml file knowing the structure in advance

    // Open the YAML file
    let file_path = "./src/foo2.yaml";
    let mut file = File::open(file_path).expect("Failed to open the file.");

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read the file.");

    // Parse the yaml into your custom data structure
    let data: MyData = serde_yaml::from_str(&contents).expect("Failed to parse YAML.");

    println!("From foo2: {:?}", data);

    // ------------------------------------------------------------------------
    // An example using serde_yaml NOT knowing the structure in advance
    let file_path = "./src/foo3.yaml";
    let mut file = File::open(file_path).expect("Failed to open the file.");

    // Parse the YAML file into a serde_yaml::Value object
    let yaml_content: serde_yaml::Value =
        serde_yaml::from_reader(file).expect("Failed to parse YAML.");

    // Now you can work with the parsed YAML data
    // The structure can be inspected and accessed dynamically using methods provided by serde_yaml::Value

    // Example: Print the parsed YAML
    println!("{:#?}", yaml_content);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct MyData {
    // Define the structure of your data
    // based on the YAML file's structure
    key1: String,
    key2: u32,
}
