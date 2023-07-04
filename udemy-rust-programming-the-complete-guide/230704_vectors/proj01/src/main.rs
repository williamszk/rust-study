use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("Hello, world!");
    let mut nums: Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let popped_num = nums.pop(); // Option<T> will return None or Some(T)

    println!("{:?}", popped_num);

    let two = nums[1]; // copy

    println!("{:?}", two);

    let one = nums.first();

    println!("{:?}", one); // return Option

    // nums.last();
    // nums.first_mut and .last_mut will borrow mutable reference

    let length = nums.len();
    println!("{}", length);

    let is_empty = nums.is_empty();
    println!("{}", is_empty);

    // insert at the index of choice
    // this will mutate the vector
    nums.insert(1, 99);
    nums.insert(2, 92);
    nums.insert(3, 93);
    println!("{:?}", nums);

    let _ = nums.remove(1);
    println!("{:?}", nums);

    nums.sort();
    println!("sorted: {:?}", nums);

    nums.reverse();
    println!("reversed: {:?}", nums);

    nums.shuffle(&mut thread_rng());
    println!("shuffled: {:?}", nums);


}
