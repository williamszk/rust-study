
rm -rf proj01

cargo new proj01 --lib
cd proj01
cargo test
# cargo run

# By default the tests are ran in parallel
# but we can change the number of threads that are used
cargo test -- --test-threads=4
# If the tests are light then this will not make a huge difference

# We can run just one test, to not run all tests, we want to see just one test
cargo test test04

# to run a subset of tests
cargo test it
# where "it" is the beginning of the name of the test
# it could be any combination


