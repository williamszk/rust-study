fn main() {
    println!("Hello, world!");
    let my_var = MyStruct { x: vec![1] };

    println!("{:?}", my_var);
    let my_var = another_function(my_var);
    println!("{:?}", my_var);
}

fn another_function(mut passing_through: MyStruct) -> MyStruct {
    let x = vec![11, 12, 13];
    passing_through.x = x;
    passing_through
}

#[derive(Debug)]
struct MyStruct {
    x: Vec<i64>,
}
