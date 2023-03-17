
run:
	rm src/c_code/program_01.o &
	rm src/c_code/program_02.o &
	rm src/c_code/libprogram.a &
	gcc -c src/c_code/program_01.c -o src/c_code/program_01.o 
	gcc -c src/c_code/program_02.c -o src/c_code/program_02.o
	ar rcs src/c_code/libprogram.a src/c_code/program_01.o src/c_code/program_02.o
	cargo run