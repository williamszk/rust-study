fn main() {
    // x <- 1:2
    // x <- x*10
    // x <- log(x)
    // x <- sum(x)
    // x <- exp(x)
    // x
    // 200

    let sum_result: f64 = vec![1_f64, 2_f64]
        .iter()
        .map(|x| (x * 10_f64).ln())
        .sum::<f64>()
        .exp();

    println!("{:?}", sum_result); // 200.0000000000001

    // given to some rouding issues the value is not exactly 200.00 which is the right answer
    // also we could do all operations in the same line
    // without the need to declare new variables in the middle of the operations

    // in R this same operation would be:
    // exp(sum(log((1:2)*10)))
    // which I think is a little harder to read

    // the syntax of using method chaning is more pleasant
    // does other FP implement this same kind of operations?
    // how would we write this in Go?
}
