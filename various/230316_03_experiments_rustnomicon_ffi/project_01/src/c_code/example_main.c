#include <stdio.h>
#include <stdint.h>
#include "program.h"

void print_arr(uint8_t *arr, int len)
{
    printf("[");
    for (int i = 0; i < len; ++i)
    {
        printf("%d ", arr[i]);
    }
    printf("]\n");
}

int main()
{
    printf("Hello World!\n");
    int var1 = 0;
    printf("[bf] var1: %d\n", var1);
    int *var1_ptr = &var1;
    uint8_t var2[16] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    printf("[bf] var2: ");
    print_arr(var2, 16);

    my_func_c_01(var1_ptr, var2);

    printf("[af] var1: %d\n", var1);
    printf("[af] var2: ");
    print_arr(var2, 16);

    printf("===============================================================\n");
    printf("Experiment with passing a struct to be filled up\n");
    uint8_t my_field2[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    MyStruct my_struct = {
        .field1 = 0,
        .field2 = my_field2,
    };


    printf("[bf] my_field1: %d\n", my_struct.field1);
    printf("[bf] my_field2:");
    print_arr(my_struct.field2, 16);

    fill_up_my_struct(&my_struct);

    printf("[af] my_field1: %d\n", my_struct.field1);
    printf("[af] my_field2:");
    print_arr(my_struct.field2, 16);
}