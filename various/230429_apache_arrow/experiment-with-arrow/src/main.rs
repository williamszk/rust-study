use std::sync::Arc;

use arrow::array::{Array, Float64Array, Int32Array, Int32Builder, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
// use arrow::ipc::RecordBatch;
use arrow::error::Result as ArrowResult;
use arrow::record_batch::RecordBatch;
use rand::distributions::{Distribution, Uniform};

fn main() -> ArrowResult<()> {
    println!("Hello, world!");

    let my_array = Int32Array::from(vec![Some(1), None, Some(2), Some(3)]);
    // assert_eqarrow::array::array_primitive::PrimitiveArray
    // fn from(data: Vec<ArrowPrimitiveType<Self = Int32Type>::Native>) -> Self!(my_array.len(), 4);
    assert_eq!(my_array.len(), 4);
    assert_eq!(my_array.value(0), 1);
    assert_eq!(my_array.is_null(1), true);

    println!("{:?}", my_array);

    println!("===============================================================");
    let range = Uniform::from(1..100);
    let mut rng = rand::thread_rng();

    // initialize an array builder
    // this array_build will be used to append new values and then we can create
    // a new array out of it
    let mut primitive_array_builder = Int32Builder::new(100);

    // append to the array the numbers that were randomly generated
    // if even append it, otherwise include a null
    for _ in 0..50 {
        let value = range.sample(&mut rng);

        if value % 2 == 0 {
            primitive_array_builder.append_value(value).unwrap();
        } else {
            primitive_array_builder.append_null().unwrap();
        }
    }
    // finish the building of the primitive array
    let my_primitive_array = primitive_array_builder.finish();
    // we can now print its values
    println!("my_primitive_array: {:?}", my_primitive_array);
    println!("===============================================================");
    // append more values to the array builder
    // but now we append a whole vector, or slice
    let values = (0..50)
        .map(|_| range.sample(&mut rng))
        .collect::<Vec<i32>>();

    primitive_array_builder.append_slice(&values).unwrap();

    let my_primitive_array = primitive_array_builder.finish();
    println!("my_primitive_array: {:?}", my_primitive_array);
    // note that the values of the previous step in building the array are flushed out
    // then the primitive_array_build will not have the older values
    println!("===============================================================");
    // Field
    // Schema
    // RecordBatch

    // define a schema
    let schema = Schema::new(vec![
        Field::new("string", DataType::Utf8, false),
        Field::new("int", DataType::Int32, false),
        Field::new("float", DataType::Float64, false),
    ]);

    // initialize primitive arrays with values
    let string_array = StringArray::from(vec!["one", "two", "three", "four", "five"]);
    let int_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let float_array = Float64Array::from(vec![1.1, 2.2, 3.3, 4.4, 5.5]);

    // build a RecordBatch using the Schema and Fields
    let my_record_batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(string_array),
            Arc::new(int_array),
            Arc::new(float_array),
        ],
    )?;

    println!("{:?}", my_record_batch);

    // Return success (an empty tuple).
    Ok(())
}
