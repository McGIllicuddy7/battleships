#include <stdio.h>
#define USING_BLIBC
#include "../bridget-libc/blibc.h"
int main(void){
   Arena * arena = arena_create();
   i32_vec_t list = make_vec(arena,i32_vec_t);
   for(i32 i =0; i<10; i++){
        vec_push(list, i);
   }
    vec_insert(list, 10,0);
   for(size_t i =0; i<list.len; i++){
        printf("%d\n", list.data[i]);
   }

}
