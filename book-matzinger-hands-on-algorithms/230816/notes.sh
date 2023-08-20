
cargo new proj01
cd proj01
cargo run
cargo test


cargo new proj02
cd proj02
cargo run
cargo test

cargo new proj03 --lib
cd proj03
cargo test
cargo bench


# how to change the version of rust compiler ----------------------------------
# install nightly toolchain
rustup toolchain install nightly
# change the rust compiler to use the nightly version
rustup default nightly







