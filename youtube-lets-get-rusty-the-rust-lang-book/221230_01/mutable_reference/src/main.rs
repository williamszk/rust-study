fn main() {
    println!("Hello, world!");
    // About mutable and immutable references
    let my_str = String::from("some important string");

    // create two immutable references:
    let ref_1 = &my_str;
    let ref_2 = &my_str;
    println!("ref_1: {}, ref_2: {}", ref_1, ref_2);
    // we can create as many IMMUTABLE references that we want

    // Can we create IMMUTABLE references from a MUTABLE variable?
    let mut my_str2 = String::from("This string should be able to grow...");
    let ref_3 = &my_str2;
    let ref_4 = &my_str2;
    println!("Immutable references from mutable variable:");
    println!("ref_3: {}", ref_3);
    println!("ref_4: {}", ref_4);
    // the mut in my_str2 is not being used...

    // However, we can't have more than 1 mutable reference to a variable...
    let mut my_str3 = String::from("This string will be referenced by just 1 mutable reference...");

    let my_ref_1 = &mut my_str3;
    println!("my_ref_1: {}", my_ref_1);
    my_ref_1.push_str("A new string");
    println!("my_ref_1: {}", my_ref_1);

    let my_ref_2 = &mut my_str3;
    for _ in 0..5{
        my_ref_2.pop();
    }
    println!("my_ref_2: {}", my_ref_2);
    // my_ref_1.push_str("A new string"); // this will give an error

    let my_ref_3 = &mut my_str3;
    println!("my_ref_3: {}", my_ref_3);

    // A mutable reference can have just 1 owner at a time.
    // We can create sequentially different mutable references, but every time
    // that we create a new mutable reference the previous one is deleted.

    // --------------------------------------------
    // Can we have:
    // - 1 immutable reference and
    // - 1 mutable reference? 

    let mut my_mut_1 = String::from("Bloody Sunday!");

    // create an immutable reference:
    let my_ref_1 = &my_mut_1;
    println!("my_ref_1: {}", my_ref_1);

    // create a mutable reference here
    let my_ref_2 = &mut my_mut_1;
    println!("my_ref_2: {}", my_ref_2);

    // println!("my_ref_1: {}", my_ref_1); // <-- this will give error

    // ---------------------------------------------
    // Create multiple immutable references
    // And use them interchangebly
    let my_var = String::from("hi there");
    let my_ref_1 = &my_var;
    let my_ref_2 = &my_var;
    let my_ref_3 = &my_var;

    println!("my_ref_1: {}", my_ref_1);
    println!("my_ref_2: {}", my_ref_2);
    println!("my_ref_3: {}", my_ref_3);


}
