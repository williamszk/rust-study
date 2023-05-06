use plotly::common::Mode;
use plotly::{Plot, Scatter};
use rand::{self, Rng};

fn main() {
    println!("Hello, world!");

    // Create a matrix, by build a vector of vectors
    let data_00 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    // create a vector of random floats between 0 and 1
    println!("{:?}", data_00);
    let mut rng = rand::thread_rng();
    let random_floats_x: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0..1.0)).collect();
    let random_floats_y: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0..1.0)).collect();
    println!("random_float_x: {:?}", random_floats_x);
    println!("random_float_y: {:?}", random_floats_y);

    // [ ] to do: find a way to transform this uniform continuous distribution
    // into a normal distribution function

    // [ ] to do: find a way to plot the density of the random values generated

    let mut plot = Plot::new();
    let trace = Scatter::new(random_floats_x, random_floats_y).mode(Mode::Markers);
    plot.add_trace(trace);

    plot.write_html("out.html");

    // [ ] as an exercise create your own random number generator.
}
