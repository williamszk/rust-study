fn main() {
    let str1 = String::from("World");
    let str2 = str1;

    // println!("Hello, {}", str1); // this will give an error, because str1 is not the owner of the object
    println!("Hello, {}", str2);

    // can I give the object back?
    // str1 = str2;
    // maybe I can if the variable is immutable
    // println!("Hello, {}", str1);

    let mut str3 = String::from("Mars");
    let str4 = str3;

    println!("Hello, {}", str4);

    // try to return the ownership of the object to str3
    str3 = str4;
    println!("Hello, {}", str3);
    // this works
    // variables are like people, they can have things, which are objects
    // a single object can be owned by a single variable (person)

    // some types have the Copy trait
    // which means that when we pass a = b
    // the object of "b" will be copied and owned by "a"
    //
    // for types that do not have the Copy trait we use the method .clone()
    // so that we can create a new identical object in other place in memory
    let str5 = String::from("Hi there");
    let str6 = str5.clone();
    // with clone we are cloning the object, and creating a new place in memory
    // where we can store the object
    // in this way str5 and str6 both have ownership of their own copy of the object
    println!("{}", str5);
    println!("{}", str6);

    // int, str, bool, float, chars, tuples are have the Copy trait
    // Strings, vectors, arrays do not implement the Copy trait
    // we can test the assertions above

    let my_str_01 = "hi there from a &str object which implements the Copy trait";
    let my_str_02 = my_str_01;
    println!("{}", my_str_01);
    println!("{}", my_str_02);
    // we don't have problem in using the object from my_str_01 in my_str_02

    // check if the vector implements the Copy trait
    let my_vec_01 = vec![1, 2, 3, 4];
    let my_vec_02 = my_vec_01;
    // println!("{:?}", my_vec_01); // this will give an error
    println!("{:?}", my_vec_02); // this will work

    // build an example where we clone the vector to another variable
    let my_vec_03 = vec![4, 5, 6, 7];
    let my_vec_04 = my_vec_03.clone();
    println!("my_vec_03 : {:?}", my_vec_03);
    println!("my_vec_04 : {:?}", my_vec_04);

    // how the ownership rule applies in the case of passing variables to functions?
    let my_str_01 = String::from("hello monkeys");
    print_str(my_str_01);

    // try to print it after it was passed to the function
    // println!("{}", my_str_01); // this will give error
    // the object was given to the previous function
    // we can't use anymore

    let my_str_02 = String::from("goodbye zebra");
    let my_str_03 = print_return_str(my_str_02); // in this moment my_str_02 lost owenership of the object
    // and the function is giving the owernship of the object to my_str_03
    println!("Print outside of function: {}", my_str_03);

    let mut name_01 = String::from("Robert") ;
    change_name_string(&mut name_01);


    // the type String is mutable and lives on the heap
    // but we can only change it using the mut keyword
    let mut my_str_04 = String::from("hi there");
    my_str_04.push_str(" again");


}

// ownership rules
// stack: data is of fixed size
// heap: request space, return a reference/pointer

fn print_str(x: String) {
    println!("A string: {}", x);
}

fn print_return_str(x: String) -> String {
    // note that we are returning the argument 
    println!("A string and return : {}", x);
    x
}

fn change_name_string(name: &mut String){
    // what if we exclude the &mut from the signature?
    // would we be able to change its content?
    name.push_str(" is happy");
    println!("Message: {}", name);
}

