// https://youtu.be/ygL_xcavzQ4?t=6945 
fn main() {
    println!("Hello, world!");

    let mut arr_it = [1,2,3,4];

    for val in arr_it.iter(){
        println!("{}", val);
    }

    let mut iter1 = arr_it.iter();
    println!("1st {:?}", iter1.next());

}
