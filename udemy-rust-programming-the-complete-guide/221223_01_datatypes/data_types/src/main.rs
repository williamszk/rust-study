fn main() {
    let _x: i8 = 10;

    let _y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let a_byte = b'A';
    println!("{}", a_byte);

    let _x1 = 2.0;
    let _x2: f32 = 2.0;
    let _t = true;
    let _f: bool = false;
    let _c = 'c';

    // compound types
    // tuple:
    let tup = (10, "hello", true);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let (_t1, _t2, _t3) = tup;

    // arrays
    let arr1 = [1,2,3,4];
    println!("{}", arr1[0]);

    let arr2: [f32; 3] = [1.0,2.2222,3.14];
    println!("{}", arr2[2]);

    // vectors
    // they are allocated on the heap
    let mut nums = vec![1,2,3];
    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vec2 = Vec::new();
    vec2.push("Test");
    vec2.push("String");
    println!("{:?}", vec2);

    nums.reverse();
    println!("{:?}", nums);

    // Create vector with fixed capacity
    let mut _vec3 = Vec::<i32>::with_capacity(2);
    println!("{}", _vec3.capacity());

    // Create vector using an iterator
    let vec4 : Vec<i32> = (0..5).collect();
    println!("{:?}", vec4);

    // A slice is a reference to another object, in our case a vector
    let sv: &[i32] = &vec4[2..4];
    // this is called a "fat pointer", it points to the first element and the
    // number of elements in the slice
    println!("{:?}", sv);
    // a slice is a not owning reference
    // there a difference between an "ordinary reference" and a "fat pointer"
    
    // strings
    // strings are stored on the heap
    let name = String::from("Bob Martin");
    let _course = "Rust".to_string();
    let new_name = name.replace("Bob", "Rodriguez");
    println!("{}", new_name);
    // a String is like a vector

    // we also have the string slice &str
    // the string slice will borrow values from a String object
    // string slice is not modifiable and it is a fat pointer
    // containing information about the start and the length of the object in
    // memory
    let str1 = "hello";
    let str2 = str1.to_string(); // turn &str into String
    let _str3 = &str2; // turn String into a &str
    


}
