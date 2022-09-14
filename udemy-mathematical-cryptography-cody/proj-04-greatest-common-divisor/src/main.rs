// https://www.udemy.com/course/cryptography/learn/lecture/2145274#overview
// implement greatest common divisor 

fn main() {

    println!("gcd(12157, 507) = {}", gcd(12157, 507));
    println!("gcd(123, 12) = {}", gcd(123, 12));
    println!("gcd(22,11) = {}", gcd(22,11));
}

fn gcd(a: i32, b:i32) -> i32 {

    let mut b_use = b;
    let mut a_use = a;

    let mut res;

    loop {
        res = a_use % b_use;

        if res == 0 {
            return b_use;
        } else {
            a_use = b_use;
            b_use = res;
        }
    }
}