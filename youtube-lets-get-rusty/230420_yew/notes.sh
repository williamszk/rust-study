cargo new yew_app
cd yew_app
cargo run

# install rustup compilation for wasm32
rustup target add wasm32-unknown-unknown
# info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date

cargo install trunk
# [root@06b49884674f yew_app]# cargo install trunk
#     Updating crates.io index
#      Ignored package `trunk v0.16.0` is already installed, use --force to override

trunk serve