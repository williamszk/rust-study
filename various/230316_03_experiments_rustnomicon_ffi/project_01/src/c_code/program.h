#include <stdint.h>

int my_func_c_01(int *var1, uint8_t *var2);

typedef struct MyStruct
{
    int field1;
    uint8_t *field2;
} MyStruct;

int fill_up_my_struct(MyStruct *my_struct);