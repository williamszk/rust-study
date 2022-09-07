fn main() {
    // reproduce these operations down below
    // 1. ( log( 0.01^200 ) ) // -Inf
    // 2. ( 200 * log(0.01) ) // -921.034 <- this is the correct one

    // in problem 1. there is a rounding problem, and basically we have log(0)
    // which is negative infinite
    // the floating point operation cannot handle more than ~13 of precision
    // checkout the mas of precision of float32 and float64

    // when dealing with probability we should use log of probability
    // so that we can avoid the rounding problem 

    println!("( 0.01_f64.powf(200.0_f64).ln() = {}", 0.01_f64.powf(200.0_f64).ln());
    println!("0.01_f64.ln()*200 = {}", 0.01_f64.ln()*200_f64);

}
