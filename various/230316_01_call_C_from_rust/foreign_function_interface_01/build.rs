// Those declarations will tell the cargo build/run where to get the 
// static libraries that we need
// extern crate cc;

// extern crate cc;

fn main(){
    println!("cargo:rustc-link-search=./src/c_code");
    println!("cargo:rustc-link-lib=static=num");
}