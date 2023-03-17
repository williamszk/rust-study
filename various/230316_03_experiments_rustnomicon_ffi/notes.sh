
# https://doc.rust-lang.org/nomicon/ffi.html
# The code in here is partially related to the Rustnomicon
# But we try to build types and functions of our own
cargo new project_01
cd project_01
cargo run

# ------------------------------------------------------------------------------ 
rm src/c_code/program_01.o
rm src/c_code/program_02.o
gcc -c src/c_code/program_01.c -o src/c_code/program_01.o 
gcc -c src/c_code/program_02.c -o src/c_code/program_02.o
C_INCLUDE_PATH=./src/c_code/ gcc src/c_code/program_01.o src/c_code/program_02.o -Wall src/c_code/example_main.c -o example_main.out
./example_main.out

# ------------------------------------------------------------------------------ 
rm src/c_code/program_01.o
rm src/c_code/program_02.o
rm src/c_code/libprogram.o
gcc -c src/c_code/program_01.c -o src/c_code/program_01.o 
gcc -c src/c_code/program_02.c -o src/c_code/program_02.o
ar rcs src/c_code/libprogram.a src/c_code/program_01.o src/c_code/program_01.o


