// https://github.com/LukeMathWalker/ndarray-koans/blob/master/src/koans/

#[cfg(test)]
mod greetings {
    #[test]
    /// This is your starting block!
    ///
    /// In each test, you are expected to replace __ in order to make test pass.
    ///
    /// Sometimes a one-liner (or a literal value) will be enough.
    /// Sometimes you will have to write a bit more to get the job done.
    ///
    /// If you get stuck, don't hesitate to ping me!
    fn the_beginning_of_your_journey() {
        // let i_am_ready_to_start = __;
        let i_am_ready_to_start = true;

        assert!(i_am_ready_to_start);
    }
}

#[cfg(test)]
mod constructors_from_vec {
    use ndarray::Array;

    #[test]
    // Given that `Array` is a generalization of `Vec`,
    // it's fair to expect that you can get a `Vec` and turn it into an `Array`.
    fn from_vec() {
        let vector: Vec<u32> = vec![1, 2, 7, 4];

        let ndarray_vector = Array::from(vector);

        // `.len()` returns the number of elements in an array
        assert_eq!(ndarray_vector.len(), 4);
        // You can index 1-dimensional arrays using the same notation you use for `Vec`
        assert_eq!(ndarray_vector[0], 1);
        assert_eq!(ndarray_vector[2], 7);
    }
}

#[cfg(test)]
mod constructors_macro_literal {
    use ndarray::{array, Array};

    #[test]
    // You are not forced to pass through a `Vec` to create an `Array`.
    //
    // The `array!` macro follows exactly the same syntax of the `vec!` macro
    // for 1-dimensional arrays and gives you directly an `Array` instance.
    fn macro_literal() {
        let from_vector = Array::from(vec![0, 1, 2]);
        let with_macro = array![0, 1, 2];

        assert_eq!(from_vector, with_macro);
    }
}

#[cfg(test)]
mod constructors_two_dimensional {
    use ndarray::array;

    #[test]
    // 1-dimensional arrays are cool, but you already knew how to do that with `Vec`.
    // You can use the `array!` macro to create multi-dimensional arrays as well!
    fn two_dimensional() {
        let matrix = array![[0, 1, 2], [3, 4, 5]];

        // `.ndim()` returns the number of dimensions of an array
        assert_eq!(matrix.ndim(), 2);
        assert_eq!(matrix.len(), 6);
        // Indexing a multi-dimensional arrays is slightly different:
        // you need to use square brackets to specify the sequence of indexes
        // (one for each dimension of your array).
        assert_eq!(matrix[[1, 2]], 5);
        assert_eq!(matrix[[0, 1]], 1);
    }
}

#[cfg(test)]
mod constructors_three_dimensional {
    use ndarray::array;

    #[test]
    fn three_dimensional() {
        // Can you guess how the `array!` macro generalizes to 3-dimensional arrays?
        let tensor = array![[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]]];

        assert_eq!(tensor.ndim(), 3);
        // `dim()` returns the shape of an n-dimensional array as a tuple
        assert_eq!(tensor.dim(), (3, 2, 2));
    }
}

#[cfg(test)]
mod constructors_random {
    // Quite often (especially for testing purposes) you'd like to generate an array
    // filled with random values: that's where `ndarray_rand` comes in!
    //
    // `ndarray-rand` combines `ndarray` and the `rand` crate.
    // It exports `RandomExt`, an extension trait that provides additional methods
    // to generate random `Array`s - it just needs to be in scope.
    //
    // Let's give it a spin!

    use ndarray::Array;
    // Use statements to get extensions traits in scope (`QuantileExt` for `min`/`max` and
    // `RandomExt` for random array generation)
    use ndarray_rand::RandomExt;
    use ndarray_stats::QuantileExt;
    // `ndarray_rand` re-exports the `rand` and the `rand_distr` crates as submodules.
    use ndarray_rand::rand_distr::Uniform;

    #[test]
    fn random() {
        let shape = (10, 10, 10);
        let distribution = Uniform::new(0, 10);
        let a = Array::random(shape, distribution);

        assert_eq!(a.ndim(), 3);
        // `min` and `max` are methods provided by `QuantileExt`, an extension trait
        // for `Array` exported by `ndarray-stats`.
        // `ndarray-stats` provides additional methods to do statistics using n-dimensional
        // arrays.
        assert!(*a.min().unwrap() >= 0);
        assert!(*a.max().unwrap() <= 10);
    }
}

#[cfg(test)]
mod constructors_random_take2 {
    use ndarray::Array;
    use ndarray_rand::rand_distr::Uniform;
    use ndarray_rand::RandomExt;
    // use ndarray_stats::QuantileExt;
    use std::any::Any;

    #[test]
    fn random() {
        // So far we have always trusted the compiler to infer the right element type for our
        // arrays based on our usage of them.
        // What if we wanted to be explicit and specify the element type?
        //
        // That's indeed possible, but we need to look a bit closer at how `Array` works.
        //
        // `Array` takes two type parameters:
        // - the element type `A`,
        // - a dimension type `D`.
        //
        // We'll get back to the dimension type `D` later.
        //
        // You can let the compiler infer either of the two using a single underscore.
        //
        // Replace the double underscores `__` appropriately to make sure that
        // `a` has elements of type `i32`.
        let a: Array<_, _> = Array::random((1000, 5), Uniform::new(1, 10));

        let element = a[[0, 0]];
        assert_eq!(element.type_id(), 0_i32.type_id())
    }
}

/// As specified in `greetings`, our main goal today is implementing K-means clustering.
///
/// What is it about? Wikipedia to the rescue:
///
/// ```txt
/// Cluster analysis or clustering is the task of grouping a set of objects in such a way
/// that objects in the same group (called a cluster) are more similar (in some sense)
/// to each other than to those in other groups (clusters).
/// ```
///
/// K-means is quite a popular algorithm when it comes to clustering: it tries to
/// partition observations into `k` clusters (`k`-means) minimizing the mutual
/// distance of observations belonging to the same cluster.
/// If each observation is a numerical vector, the distance is usually the euclidean distance.
///
/// In mathematical terms, it tries to minimize this loss function:
///
///  k     1
///  Σ   ―――――     Σ    ‖x-y‖²
/// i=1 2*|S_i|   x,y
///              in S_i
///
/// where `S_i` is one of the `k` clusters, `x` and `y` are observations in the `S_i` cluster.
/// Check https://en.wikipedia.org/wiki/K-means_clustering#Description if you don't like
/// unicode math formulas (rightly so).
///
#[cfg(test)]
mod cluster_generation_origin {
    use approx::assert_abs_diff_eq;
    use ndarray::{array, Array, Axis};
    use ndarray_rand::rand_distr::Normal;
    use ndarray_rand::RandomExt;

    /// Our first step in our K-means implementation journey is data generation!
    ///
    /// To spot clusters, you need to have some data first.
    /// Using what we learned in the `constructors` koan, try to generate a matrix of observations:
    /// one row for each data point.
    /// We want our observations to be normally distributed around the origin, the 0 vector.
    #[test]
    fn origin_cluster() {
        let n_observations = 10000;
        let n_features = 2;
        let a = Array::random(
            (n_observations, n_features),
            Normal::new(0.0, 1.0).expect("Had some problem with Normal struct constructor."),
        );

        // The mean point of a cluster is called `centroid`.
        // We'll use this term again when implementing the actual K-means algorithm.
        // `mean_axis` can return `None` if the axis we are reducing has length 0
        // (not our case here, we can safely use `expect` or `unwrap` to get the value).
        let centroid = a.mean_axis(Axis(0)).expect("Failed to computer mean.");
        let variance = a.var_axis(Axis(0), 1.);

        // Both `mean_axis` and `var_axis` reduce the dimensionality of the array:
        // they compute the mean and the variance along the specified axis and return a
        // new array with one less dimension (the axis you specified for reduction is removed).
        assert_eq!(centroid.ndim(), 1);
        assert_eq!(variance.ndim(), 1);
        assert_eq!(centroid.dim(), 2);
        assert_eq!(variance.dim(), 2);

        // When dealing with floats, it's not a good idea to use equality checks:
        // rounding errors affect the precision of the result, making strict equality
        // quite flaky.
        // `ndarray` provides an `approx` feature-flag to bring approximate comparisons
        // according to the traits defined in the `approx` crate:
        // `assert_abs_diff_eq` checks that absolute difference between each element
        // in the two arrays is smaller than the specified `epsilon`.
        assert_abs_diff_eq!(centroid, array![0., 0.], epsilon = 0.1);
        assert_abs_diff_eq!(variance, array![1., 1.], epsilon = 0.1);

        // (Yes, we are randomly generating `a`, hence this test is not fully deterministic,
        //  but you'd have to be quite unlucky to see it fail. I cut myself some slack here.)
    }
}

#[cfg(test)]
mod cluster_generation_as_a_function {
    use ndarray::{Array, Array2, Ix2};
    use ndarray_rand::rand_distr::Normal;
    use ndarray_rand::RandomExt;

    /// Let's isolate the code required to generate a cluster in a proper function,
    /// so that we can call it again from other tests.
    ///
    /// As we said before, `Array` takes two type parameters:
    /// - `A`, the element type;
    /// - `D`, the dimension type.
    ///
    /// We want to formalize in our function signature that the array we are returning
    /// has exactly two dimensions (thus allowing the compiler to verify for us **at compile-time**
    /// that we are not trying to do something nonsensical down the line, like summing
    /// arrays with different numbers of dimensions).
    ///
    /// We can use `Ix2` as dimension type, thus using `Array<f64, Ix2>` as our output type.
    /// Otherwise, we can leverage `ndarray`'s type aliases: `Array2<T>` is a shortcut
    /// for `Array<T, Ix2>`.
    /// As you can imagine, you can use `Array1`, `Array3`, etc. to work with a different number
    /// of dimensions.
    pub fn generate_cluster(n_observations: usize, n_features: usize) -> Array2<f64> {
        Array::random(
            (n_observations, n_features),
            Normal::new(0.0, 1.0).expect("Sorry"),
        )
    }

    #[test]
    fn as_a_function() {
        let n_observations = 10000;
        let a: Array2<f64> = generate_cluster(n_observations, 2);
        let b: Array<f64, Ix2> = generate_cluster(n_observations, 3);

        assert_eq!(a.ndim(), b.ndim())
    }
}

#[cfg(test)]
mod cluster_generation_smoke_check {
    use ndarray::Array2;
    // Let's import our generation function from the previous test module
    use super::cluster_generation_as_a_function::generate_cluster;
    use ndarray_npy::{read_npy, write_npy};

    #[test]
    /// One thing is checking with a couple of assertions that mean and variance are close
    /// to what we expect.
    ///
    /// Another thing is visually confirming that the cluster we just generated has indeed
    /// that round cloudy shape that we expect it to have.
    ///
    /// `ndarray-npy` provides two convenience functions to serialize and deserialize an
    /// array in `npy` format: `read_npy` and `write_npy`.
    ///
    /// `npy` is one of the serialization format used by Python's NumPy:
    /// Rust is still quite immature when it comes to plotting, so we'll save our array in `npy`
    /// format and leverage Python to do some plotting.
    ///
    /// You can find a plug-and-play "Cluster generation" Jupyter notebook in the `python` folder,
    /// give it a go!
    fn smoke_check() {
        let a: Array2<f64> = generate_cluster(100, 2);
        let filename = "python/cluster_smoke_check.npy";

        write_npy(filename, a.clone()).expect("Failed to write array in npy format.");
        let b: Array2<f64> = read_npy(filename).expect("Failed to read array from npy format.");

        assert_eq!(a, b);
    }
}


