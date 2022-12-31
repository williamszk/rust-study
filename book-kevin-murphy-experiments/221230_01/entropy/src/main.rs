// Section 2.8.1
// Try to calculate the entropy

use fast_math;

fn main() {

    // A discrete random variable can be represented by those two vectors.
    let p = [0.25, 0.25, 0.2, 0.15, 0.15];
    println!("Probabilities: {:?}", p);
    // let x = [1, 2, 3, 4, 5];
    // The values that the random variable assumes and the probability of 
    // each value.
    let mut acc = 0.0f32;
    for i in 0..5{
        // println!("p[i]={}, x[i]={}", p[i], x[i]);
        // let x_f32 = x[i] as f32;
        // https://docs.rs/fast-math/latest/fast_math/fn.log2.html
        let elem = p[i]*fast_math::log2_raw(p[i]);
        acc += elem;
    }
    let entropy = -1.0f32  * acc;
    println!("Entropy: {}", entropy);
}
