fn main() {
    println!("Hello, world!");

    say_hello();

    get_sum(10, 2);

    println!("{}", get_sum_2(10, 3));
    println!("{}", get_sum_3(10, 5));

    println!("{:?}", get_2(10));

    // an example of enpacking when the function returns a tuple
    let (val01, val02) = get_2(20);
    println!("{}, {}", val01, val02);


    let my_list = vec![1,2,3,4];
    println!("Sum of items in list: {}", sum_list(&my_list));
}

fn say_hello() {
    println!("Hello");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    // println!("{} + {} = {}", x, y, x + y);
    x + y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    // println!("{} + {} = {}", x, y, x + y);
    return x + y;
}

// how to return a tuple?
fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

// how to pass a vector as argument to a function
fn sum_list(list: &[i32]) -> i32 {
    // how do I pass an array to a function?
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    // is there a more declarative way to sum all elements of the vector?
    // like a reduce function? or just a sum method

    sum
}