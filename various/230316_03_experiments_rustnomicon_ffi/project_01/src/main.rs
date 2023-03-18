
extern "C"{
    fn my_func_c_01(var1: *mut i32, var2: *mut u8) -> i32;


    // int fill_up_my_struct(MyStruct *my_struct);

    fn fill_up_my_struct(my_struct: *mut MyStruct) -> i32;
}

#[repr(C)]
struct MyStruct{
    field1: i32,
    field2: *mut u8,
}

fn main() {
    println!("Hello, world!");

    let mut var1 = 0;
    let mut var2 = Vec::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let var2_p = var2.as_mut_ptr();

    println!("[bf] var1: {}", var1);
    println!("[bf] var2: {:?}", var2);
    unsafe {
        let result = my_func_c_01(&mut var1, var2_p);
        println!("result: {}", result);
        println!("[af] var1: {}", var1);
        println!("[bf] var2: {:?}", var2);
    }
    println!("================================================================");
    println!("Experiment with passing a struct to be filled by the C function");

    let mut arr_to_be_filled_up = Vec::from([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    let arr_to_be_filled_up_p = arr_to_be_filled_up.as_mut_ptr();

    let mut my_struct = MyStruct{
        field1:0,
        field2: arr_to_be_filled_up_p,
    };

    println!("[bf] my_struct.field1: {}", my_struct.field1);
    println!("[bf] arr_to_be_filled_up: {:?}", arr_to_be_filled_up);
    unsafe {
        fill_up_my_struct(&mut my_struct);
    }
    println!("[af] my_struct.field: {}", my_struct.field1);
    println!("[af] arr_to_be_filled_up: {:?}", arr_to_be_filled_up);





}
