#include <string.h>
#include "program.h"

int fill_up_my_struct(MyStruct *my_struct){
    int to_fill_field1 = 777; 
    uint8_t to_fill_field_2[] = {7,7,7,7,7,7,7,7,7,7,7,7,7,7,7,7};
    my_struct->field1 = to_fill_field1;
    memcpy(my_struct->field2, to_fill_field_2, 16);
    return 0;
}