#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include "program.h"

/*
* var2 is a vector of bytes
*/
int my_func_c_01(int *var1, uint8_t *var2){
    *var1 = 10;
    uint8_t src_arr[] = {7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7};
    memcpy(var2, src_arr, 16);
    return 0;
}