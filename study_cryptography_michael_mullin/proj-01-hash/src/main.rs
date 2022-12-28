// in this program we will be working with this video:
// https://www.youtube.com/watch?v=pmBxPUaIvaI&ab_channel=MichaelMullin

use anyhow::{Result, bail};

fn main() {
    println!("Hello, world!");
}


// 256 bits for SHA256 is 32 bytes
// this function receives a file and outputs the digest
// Result is an enum, it have two variants: Ok and Err
// I think that [u8; 32] means it is an array of u8 with 32 elements
pub fn hash_file(file_name: &str) -> Result<[u8; 32]> {
    // what bail does?
    bail!("not implemented!")
}
