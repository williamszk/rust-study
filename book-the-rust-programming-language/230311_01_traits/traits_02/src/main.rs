// Solution without using Copy trait inside the generic type T


fn largest<T: PartialOrd+Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for &ref item in list.iter() {
        if item > largest {
            largest = item.clone();
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
