fn main() {
    let _a = println!("Hello, world!");
    // println!("a = {}", a);

    // this is not possible because "let" creates a statement
    // let _c = let _b = 10;
    let _b = 10;
    // this is not possible
    // let _c = _b = 11;
    // assignment is a statement too

    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {}", y);
    
}

// function definitions are statements we cannot assign it to a variable
// let a = fn hello(){
//     "hello"
// }
