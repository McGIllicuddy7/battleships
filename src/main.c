#include <stdio.h>
#define USING_BLIBC
#include "../bridget-libc/blibc.h"
blibc_enable_hash_map(int, str_t, int_str_hash_map)
static u64 hash_int(int a){
     return (u64)(a);
}

static bool int_eq(int a, int b){
     return a == b;
}

static void destroy_int(int a){
     (void)a;
}

static void destroy_str(str_t a){
     (void)a;
}


int main(void){
     arena_t * arena = arena_create();
     int_str_hash_map* map = int_str_hash_map_create(hash_int, int_eq, destroy_int, destroy_str, 100);
     int count =0;
     for(i32 i =0; i<400; i++){
          int_str_hash_map_insert(map, i, str_fmt(arena, "hello :%d",i));
     }
     int_str_hash_map_iterator_t iter = int_str_hash_map_begin_iter(map);
     int_str_hash_map_key_value_pair_t * c = 0;
     int i =0;
     while((c = int_str_hash_map_iter_next(&iter))){
          printf("i:%d,<key:%d value:" STR_FMT ">\n", i,c->key, STR_ARG((c->value)))
          ;
          i+=1;
     }
     printf("count:%d\n", count);
     int_str_hash_map_destroy(map);
     arena_destroy(arena);
     debug_alloc_free_counts();
}
