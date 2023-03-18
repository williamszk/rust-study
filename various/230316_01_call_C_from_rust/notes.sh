# Code based on:
# https://dev.to/xphoniex/how-to-call-c-code-from-rust-56do

# -----------------------------------------------------------------------------  
cargo new foreign_function_interface_01
cd foreign_function_interface_01
cargo run # this fails

# -----------------------------------------------------------------------------  
gcc -c src/num.c
gcc -shared num.o -o libnum.so
rustc -l num -L. src/main.rs
./main # this will give an error

LD_LIBRARY_PATH=. ./main # this works!

# -----------------------------------------------------------------------------  
rm libnum.a
rm libnum.so
rm num.o
rm main

# -----------------------------------------------------------------------------  
# The alternative is to link the archive file (static library)
# into the rust executable
gcc -c src/num.c
ar rcs libnum.a num.o
rustc -l static=num -L. src/main.rs
./main # this works!

# -----------------------------------------------------------------------------  
gcc -c src/c_code/num.c -o src/c_code/num.o
ar rcs src/c_code/libnum.a src/c_code/num.o
cargo run

rm src/c_code/libnum.a
rm src/c_code/num.o
rm main














