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
     int * p1 = p;
     *p1 += 1;
     printf("%d "STR_FMT"\n", a, STR_ARG(*s));
}
int main(void){
     arena_t * arena = arena_create();
     int_str_hash_map* map = int_str_hash_map_create(hash_int, int_eq, destroy_int, destroy_str, 100);
     int count =0;
     for(i32 i =0; i<10000; i++){
          int_str_hash_map_insert(map, i, str_fmt(arena, "hello :%d",i));
     }
     int_str_hash_map_for_each(map, print_pair, &count);
     printf("count:%d\n", count);
     int_str_hash_map_destroy(map);
     arena_destroy(arena);
     debug_alloc_free_counts();
}
