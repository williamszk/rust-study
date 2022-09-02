fn main() {
    // https://www.youtube.com/watch?v=ygL_xcavzQ4&t=3355s&ab_channel=DerekBanas

    let vec01 : Vec<i32> = Vec::new();

    println!("{:?}", vec01);

    let mut vec02 = vec![1,2,3,4];


    vec02.push(5);

    println!("1st : {}", vec02[0]);

    // why do we need to have "second" of type
    // reference to i32? i.e. (&i32)
    // let second: &i32 = &vec02[1];

    // I understand that match is like a switch
    match vec02.get(1) {
        // I don't know what is happening here
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec02 {
        *i *= 2;
        // why do we need to use *=?
        // and why we use *i?
        // this is taking each element and multiplying it by 2
    }
    for i in &vec02 {
        println!("{}", i)
    }

    // get the length of the vector
    println!("Vec length {}", vec02.len());
    // pop the last value of vector
    println!("Pop : {:?}", vec02.pop()); // Pop : Some(10)
    // why is there a "Some"
    // what "Some" does?

    // vectors are of variable length
    // are there arrays in rust? yes we have
    // maybe vectors are more like slices in Go
    // arrays are of fixed size



}

// Next functions:
// https://youtu.be/ygL_xcavzQ4?t=3629