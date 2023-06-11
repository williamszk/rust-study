
# Modify the solution to the Section 4 assignment by creating a Trait that has 
# the set_mpg, set_color, and set_top_speed methods. Then create a Motorcycle 
# struct with the same fields as the Car struct: mpg, color, and top_speed. 
# Now implement your Trait on both the Car and Motorcycle struct. Print out the 
# results to confirm a working solution.

# Create a simple print function that uses Generic T. This Generic T will need 
# to implement std::fmt::Debug depending on the values you pass in. Our function 
# takes one parameter of type T. Our function will then print out the value that 
# is passed in.

cargo new proj
cd proj
cargo run
