#include "blibc.h"
#include <stdio.h>
#define PAGE_SIZE 4096

typedef struct BLIBC_Arena  {
    uint8_t * buffer;
    size_t offset;
    size_t capacity;
    struct BLIBC_Arena * next;
}BLIBC_Arena_t;



BLIBC_Arena_t * blibc_arena_create(void){
    BLIBC_Arena_t * out = malloc(sizeof(BLIBC_Arena_t)+PAGE_SIZE*16);
    assert(sizeof(BLIBC_Arena_t)<=64);
    uint8_t * buffer = (uint8_t*)out+64;
    out->buffer = buffer;
    out->capacity = 4096*16;
    out->next =0;
    out->offset = 0;
    return out;
}

BLIBC_Arena_t * blibc_arena_sized(size_t page_count){
    BLIBC_Arena_t * out = malloc(sizeof(BLIBC_Arena_t)+PAGE_SIZE*page_count);
    assert(sizeof(BLIBC_Arena_t)<=64);
    uint8_t * buffer = (uint8_t*)out+64;
    out->buffer = buffer;
    out->capacity = page_count;
    out->next =0;
    out->offset = 0;
    return out; 
}

void blibc_arena_destroy(BLIBC_Arena_t * arena){
    if(!arena){
        return;
    }
    blibc_arena_destroy(arena->next);
    free(arena);
}

void * blibc_arena_alloc(BLIBC_Arena_t * arena, size_t count_bytes){
    if((count_bytes%16) != 0){
        count_bytes += 16-count_bytes%16;
    }
    if(arena->offset+count_bytes<arena->capacity){
        uint8_t * current = arena->buffer+arena->offset;
        arena->offset+= count_bytes;
        return (void*)(current);
    }else{
        if (arena->next){
            return blibc_arena_alloc(arena->next, count_bytes);
        }else{
            size_t new_cap = (count_bytes/PAGE_SIZE)*PAGE_SIZE;
            if (new_cap<arena->capacity){
                new_cap = (arena->capacity/PAGE_SIZE)*PAGE_SIZE;
            }
            size_t page_count = new_cap/PAGE_SIZE;
            arena->next = blibc_arena_sized(page_count);
            return blibc_arena_alloc(arena->next, count_bytes);
        }        
    }
}

void blibc_check_bounds(size_t count, size_t at){
    assert(at<count);
}

void * blibc_arena_realloc(BLIBC_Arena * arena,void * ptr, size_t old_size, size_t count_bytes){
    if(old_size>=count_bytes){
        return ptr;
    }else{
        void * out = blibc_arena_alloc(arena, count_bytes);
        memcpy(out, ptr, old_size);
        return out;
    }
}
