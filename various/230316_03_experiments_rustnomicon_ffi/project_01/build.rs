fn main(){
    println!("cargo:rustc-link-search=./src/c_code");
    println!("cargo:rustc-link-lib=static=program");
}