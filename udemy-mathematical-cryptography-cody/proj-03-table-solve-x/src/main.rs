// In this project we take the last one proj-02 and try to build a general
// way to build a table that can receive equations and functions
// to fill up the values that are solutions to the problem
// of linear congruence

fn main() {


    // string
    // function 

    // my vector of Column info
    let columns:Vec<Column> = vec![
        Column{
            equation: String::from("2x ≡ 2 (mod 4)") ,
            function: |x:i32| -> bool {true},
        },
        // Column{

        // }
    ];


}


struct Column{
    equation: String,
    function: fn(x:i32) -> bool,
}

// how to use a function as a field of a struct in rust?
// https://stackoverflow.com/questions/37370120/right-way-to-have-function-pointers-in-struct
