#![feature(test)]
extern crate test;

/// # A new Section
/// this [markdown](https://daringfireball.net/projects/markdown/) is picked up by `Rustdoc`

/// # A Simple Addition
///
/// Adds two integers.
///
/// # Arguments
///
/// - *a* the first term, needs to be i32
/// - *b* the second term, also a i32
///
/// ## Returns
/// The sum of *a* and *b*.
///
/// # Panics
/// The addition is not done safely, overflows will panic!
///
/// # Examples
///
/// ```rust
/// use proj03;
/// assert_eq!(proj03::my_add(1, 1), 2);
/// ```
pub fn my_add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let result = my_add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn this_does_not_work() {
        assert_eq!(my_add(std::i32::MAX, std::i32::MAX), 0);
    }

    #[bench]
    fn how_fast(b: &mut Bencher) {
        b.iter(|| my_add(42, 42));
    }
}
