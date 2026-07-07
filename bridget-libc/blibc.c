#include "blibc.h"
#include <stdio.h>
#include <stdarg.h>
#include <ctype.h>
#define PAGE_SIZE 4096

typedef struct blibc_arena_t  {
    uint8_t * buffer;
    size_t offset;
    size_t capacity;
    struct blibc_arena_t * next;
}blibc_private_arena_t;



blibc_arena_t * blibc_arena_create(void){
    blibc_private_arena_t * out = mem_alloc(sizeof(blibc_private_arena_t)+PAGE_SIZE*16,1);
    assert(sizeof(blibc_private_arena_t)<=64);
    uint8_t * buffer = (uint8_t*)out+64;
    out->buffer = buffer;
    out->capacity = PAGE_SIZE*16;
    out->next =0;
    out->offset = 0;
    return out;
}

blibc_arena_t * blibc_arena_sized(size_t page_count){
    blibc_private_arena_t * out = mem_alloc(sizeof(blibc_private_arena_t)+PAGE_SIZE*page_count,1);
    assert(sizeof(blibc_private_arena_t)<=64);
    uint8_t * buffer = (uint8_t*)out+64;
    out->buffer = buffer;
    out->capacity = page_count*PAGE_SIZE;
    out->next =0;
    out->offset = 0;
    return out; 
}

void blibc_arena_destroy(blibc_private_arena_t * arena){
    if(!arena){
        return;
    }
    blibc_arena_destroy(arena->next);
    mem_free(arena);
}

void * blibc_arena_alloc(blibc_private_arena_t * arena, size_t count_bytes){
    if((count_bytes%16) != 0){
        count_bytes += 16-count_bytes%16;
    }
    if(arena->offset+count_bytes<arena->capacity-16){
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
            if (page_count<8){
                page_count =8;
            }
            arena->next = blibc_arena_sized(page_count);
            return blibc_arena_alloc(arena->next, count_bytes);
        }        
    }
}

void blibc_check_bounds(size_t count, size_t at){
    assert(at<count);
}

void * blibc_arena_realloc(blibc_private_arena_t * arena,void * ptr, size_t old_size, size_t count_bytes){
    if(old_size>=count_bytes){
        return ptr;
    }else{
        void * out = blibc_arena_alloc(arena, count_bytes);
        memcpy(out, ptr, old_size);
        return out;
    }
}

blibc_str_t blibc_str_concat(blibc_arena_t * arena, blibc_str_t s1, blibc_str_t s2){
    blibc_str_t out = {0};
    out.len = s1.len+s2.len;
    out.items = blibc_arena_alloc(arena, out.len);
    for(size_t i =0; i<s1.len; i++){
        out.items[i] = s1.items[i];
    }
    for(size_t i = 0; i<s2.len; i++){
        out.items[i+s1.len] = s2.items[i];
    }
    return out;
}

blibc_str_t blibc_str_push(blibc_arena_t * arena, blibc_str_t st, char c){
    blibc_str_t out = {0};
    out.len = st.len+1;
    out.items = blibc_arena_alloc(arena, out.len);
    memcpy(out.items, st.items, st.len);
    out.items[out.len-1] = c;
    return out;
}

blibc_str_t blibc_str_fmt(blibc_arena_t * arena, const char * fmt, ...){
    va_list args;
    va_start(args, fmt);
    size_t count = vsnprintf(0, 0, fmt, args);
    va_end(args);
    va_start(args, fmt);
    char * buffer = blibc_arena_alloc(arena, count+1);
    size_t count2 = vsnprintf(buffer, count+1, fmt, args);
    va_end(args);
    assert(count == count2);
    return (blibc_str_t){.items = buffer, .len = count};
}

blibc_str_vec_t blibc_str_split_by(blibc_arena_t * arena, blibc_str_t str, char delim){
    blibc_str_vec_t out = BLIBC_MAKE_VEC(arena, blibc_str_vec_t);
    size_t idx =0;
    blibc_str_t current;
    current.items = str.items;
    current.len = 0;
    while(idx<str.len){
        if(str.items[idx] == delim){
            if(current.len>0){
                BLIBC_VEC_PUSH(out, current);
            }
            current.items = str.items+idx+1;
            current.len =0;
        }else{
            current.len+=1;
        }
        idx += 1;
    }
    if(current.len>0){
        BLIBC_VEC_PUSH(out, current);
    }
    return out;
}

blibc_str_vec_t blibc_str_split_whitespace(blibc_arena_t * arena, blibc_str_t str){
    blibc_str_vec_t out = BLIBC_MAKE_VEC(arena, blibc_str_vec_t);
    size_t idx =0;
    blibc_str_t current;
    current.items = str.items;
    current.len = 0;
    while(idx<str.len){
        if(isspace(str.items[idx])){
            if(current.len>0){
                BLIBC_VEC_PUSH(out, current);
            }
            current.items = str.items+idx;
            current.len =0;
        }else{
            current.len+=1;
        }
        idx+=1;
    }
    if(current.len>0){
        BLIBC_VEC_PUSH(out, current);
    }
    return out;
}

char * blibc_str_to_c_string(blibc_arena_t * arena, blibc_str_t st){
    char * out = blibc_arena_alloc(arena, st.len+1);
    memcpy(out, st.items, st.len);
    out[st.len] = 0;
    return out;
}

blibc_str_t blibc_read_file_to_string(blibc_arena_t * arena, blibc_str_t filename){
    FILE * f = fopen(blibc_str_to_c_string(arena, filename), "rb");
    if(!f){
        return (blibc_str_t){0};
    }
    fseek (f, 0, SEEK_END);
    size_t length = ftell (f);
    fseek (f, 0, SEEK_SET);
    char* buffer = blibc_arena_alloc(arena,length);
    fread (buffer, 1, length, f);
    fclose (f);
    blibc_str_t out = (blibc_str_t){.items = buffer, .len= length};
    return out;
}

int blibc_write_string_to_file(blibc_str_t filename, blibc_str_t str){
    char * c_filename = blibc_str_to_c_string(0, filename);
    FILE * f = fopen(c_filename, "wb");
    if(!f){
        mem_free(c_filename);
        return -1;
    }
    size_t out = fwrite(str.items, 1, str.len,f);
    fclose(f);
    return (int)out;
}

blibc_str_t blibc_str_trim(blibc_str_t st){
    blibc_str_t out = st;
    if(!out.items){
        return out;
    }
    while(out.len>0){
        if(isspace(*out.items)){
            out.items+=1;
            out.len-=1;
        }else{
            break;
        }
    }
    while(out.len>0){
        if(isspace(out.items[out.len-1])){
            out.len-=1;
        }else{
            break;
        }
    }
    return out;
}

u64 allocation_count = 0;
u64 free_count =0;

void * blibc_debug_alloc(size_t count, size_t size){
    allocation_count +=1;
    return calloc(count, size);
}

void blibc_debug_free(void * ptr){
    free_count += 1;
    free(ptr);
}

void debug_alloc_free_counts(void){
    printf("allocations:%zu, frees:%zu\n", (size_t)allocation_count, (size_t)free_count);
    assert(allocation_count == free_count);
}

void print_alloc_free_counts(void){
    printf("allocations:%zu, frees:%zu", (size_t)allocation_count, (size_t)free_count);
}
