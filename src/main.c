#include <stdio.h>
#define USING_BLIBC
#include "../bridget-libc/blibc.h"
blibc_enable_hash_map(int, str_t, int_str_hash_map)
u64 hash_int(int a){
     return (u64)(a);
}

bool int_eq(int a, int b){
     return a == b;
}

void destroy_int(int a){
     (void)a;
}

void destroy_str(str_t a){
     (void)a;
}

void print_pair(int a, str_t * s, void *p){
     (void)p;
     printf("%d "STR_FMT"\n", a, STR_ARG(*s));
}
int main(void){
     arena_t * arena = arena_create();
     int_str_hash_map* map = int_str_hash_map_create(hash_int, int_eq, destroy_int, destroy_str, 10);
     for(i32 i =0; i<40; i++){
          int_str_hash_map_insert(map, i, str_fmt(arena, "hello :%d",i));
     }
     int_str_hash_map_for_each(map, print_pair, 0);
}
