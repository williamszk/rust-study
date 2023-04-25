# https://www.youtube.com/watch?v=lJllt5X6ELg&list=PLFjq8z-aGyQ6t_LGp7wqHsHTYO-pDDx84&index=32&ab_channel=CodingTech
# https://youtu.be/lJllt5X6ELg?t=348

# to serve the web app through the browser
cargo install trunk

rustup target add wasm32-unknown-unknown

cargo new yew-video-streaming
cd yew-video-streaming

touch index.html style.css

# =============================================================================
# to run the yew server
cd yew-video-streaming
trunk serve
