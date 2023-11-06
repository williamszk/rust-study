pub fn _main() {
    println!("Calling from enums mod.");
    _different_types();
    _experiment_trait_inside_vector();
}

fn _different_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn _experiment_trait_inside_vector() {
    // When you’re writing a program, if you don’t know the exhaustive set of
    // types the program will get at runtime to store in a vector, the enum tech-
    // nique won’t work. Instead, you can use a trait object, which we’ll cover in
    // Chapter 17.

    trait Walkable {
        fn walk(&self);
    }

    impl std::fmt::Debug for dyn Walkable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "This is a trait object: '{}'", "This is mutable")
        }
    }

    #[derive(Clone, Debug)]
    struct Dog<'a> {
        name: &'a str,
    }

    impl Walkable for Dog<'_> {
        fn walk(&self) {
            println!("{} the dog is walking...", self.name);
        }
    }

    let bob = Dog { name: "Bob" };
    bob.walk();

    // ----------------------------------------------------------------------
    #[derive(Clone, Debug)]
    struct Cat<'a> {
        name: &'a str,
    }

    impl Walkable for Cat<'_> {
        fn walk(&self) {
            println!("{} the cat is walking...", self.name);
        }
    }

    let fifi = Cat { name: "Fifi" };

    // this is how we create a vector of traits
    let mut v_walkable: Vec<Box<dyn Walkable>> = Vec::new();

    v_walkable.push(Box::new(bob.clone()));
    v_walkable.push(Box::new(bob.clone()));
    v_walkable.push(Box::new(fifi.clone()));

    let mut v_walkable_iter = v_walkable.iter();
    while let Some(element) = v_walkable_iter.next() {
        println!("{:?}", element);
    }
}

// ----------------------------------------------------------------------
