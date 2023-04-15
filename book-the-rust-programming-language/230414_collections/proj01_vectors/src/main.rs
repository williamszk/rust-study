fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    println!("{:?}", v);

    let v = vec![1, 2, 3];

    println!("{:?}", v);
    println!("----------------------------------------------------------");

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);
    println!("----------------------------------------------------------");

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // this gives an error
    let does_not_exist = v.get(100);

    println!("{:?}", does_not_exist);

    println!("----------------------------------------------------------");
    let mut v = vec![1, 2, 3, 4, 5];
    {
        let first = &v[0];
        println!("The first element is: {}", first);
    }
    v.push(6);

    println!("----------------------------------------------------------");
}
