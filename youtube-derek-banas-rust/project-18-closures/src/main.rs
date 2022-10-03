// https://youtu.be/ygL_xcavzQ4?t=7093
fn main() {
    println!("Hello, world!");
    // Maybe in here is talking about anonymous functions
    // that are then assigned to variables

    let can_vote = |age: i32| age >= 18;

    println!("Can vote: {}", can_vote(8));

    // closures can access values that are outside of its body

    let mut sample1 = 5;

    let print_var = || println!("sample1 = {}", sample1);

    print_var();
    // I think it is because the function is defined inside the same context as sample1

    sample1 = 10;

    let mut change_var = || sample1 += 1;

    change_var();

    println!("sample1 = {}", sample1);

    sample1 = 10;

    println!("sample1 = {}", sample1);

    // This example shows how we can change the values of variables inside closures

    // we can create functions inside our main function
    // they will act like closures in this case

    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b)
    }

    let sum = |a: i32, b: i32| a + b;
    let prod = |a: i32, b: i32| a * b;

    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("3 * 4 = {}", use_func(3, 4, prod));
}

// about smart pointers
// https://youtu.be/ygL_xcavzQ4?t=7551