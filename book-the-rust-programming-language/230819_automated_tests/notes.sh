
cargo new proj01
cd proj01
cargo run


# When we include #[test] in a function
# does this new source code affect the size of the final executable?
# it shouldn't code test shouldn't affect the final executable


cargo new proj02 --lib
cd proj02
cargo run


cargo new addr --lib
cd addr

cargo test

# how to create a test module?
# I think we can just include a `mod test{}` 

# Doc-tests: tests in documentation. `cargo-test` will run the tests that are
# specified in the documentation.









