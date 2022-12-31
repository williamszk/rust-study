// https://stackoverflow.com/a/32872974/8782077

fn main() {
    let my_vec_01 = vec![1, 2];

    // how to multiply all elements from a vector
    let my_vec_02: Vec<i32> = my_vec_01.iter().map(|x| x * 10).collect();
    println!("my_vec_02: {:?}", my_vec_02);

    // we could do it in another way
    let mut my_vec_03 = vec![1, 2];
    my_vec_03.iter_mut().for_each(|x| *x *= 10);
    println!("my_vec_03: {:?}", my_vec_03);

    // we could also do for loops to achieve the same result
    let mut my_vec_04 = vec![1, 2];
    for i in &mut my_vec_04 {
        *i *= 10;
    }
    println!("my_vec_04: {:?}", my_vec_04);

    // we could also build a vector using range
    let my_vec_05: Vec<i32> = (1..11).collect();
    println!("my_vec_05: {:?}", my_vec_05);
}
