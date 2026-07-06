#ifndef B_LIBC_H
#define B_LIBC_H
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>
#include <string.h>

#define BLIBC_IMPROVE_TYPE(Type, Name)\
    typedef struct {\
        Type value;\
        bool is_valid;\
    }Name##_opt_t;\
    \
    typedef struct{\
        Type * data;\
        size_t len;\
    }Name##_slice_t;\
    \
    typedef struct {\
        struct BLIBC_Arena * arena;\
        Type * data;\
        size_t len;\
        size_t cap;\
    }Name##_vec_t;\
    \
    typedef struct {\
        Type * ptr;\
        void (*destructor)(Type *);\
    }Name##_owned_t;\
    \
    typedef struct {\
        Type * ptr;\
        size_t * ref_count;\
        void (*destructor)(Type *);\
    }Name##_rc_t;\

    
#define BLIBC_GET(list, at) (*((blibc_check_bounds(list.len, at)),&list.data[at]))

#define BLIBC_MAKE_SLICE(Type, count) ((Type){.data =calloc(sizeof(((Type*)(64))->data[0]),count), .len = count});

struct BLIBC_Arena;
typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;
typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;
typedef float f32;
typedef double f64;

BLIBC_IMPROVE_TYPE(i8, i8)
BLIBC_IMPROVE_TYPE(i16, i16)
BLIBC_IMPROVE_TYPE(i32, i32)
BLIBC_IMPROVE_TYPE(i64, i64)
BLIBC_IMPROVE_TYPE(u8, u8)
BLIBC_IMPROVE_TYPE(u16, u16)
BLIBC_IMPROVE_TYPE(u32, u32)
BLIBC_IMPROVE_TYPE(u64,u64)
BLIBC_IMPROVE_TYPE(f32, f32)
BLIBC_IMPROVE_TYPE(f64, f64)
BLIBC_IMPROVE_TYPE(bool, bool)
void blibc_check_bounds(size_t count, size_t at);
typedef struct BLIBC_Arena BLIBC_Arena;
BLIBC_Arena * blibc_arena_create(void);
BLIBC_Arena * blibc_arena_sized(size_t page_count);
void blibc_arena_destroy(BLIBC_Arena * arena);
void * blibc_arena_alloc(BLIBC_Arena * arena, size_t count_bytes);
void * blibc_arena_realloc(BLIBC_Arena * arena,void * ptr, size_t old_size, size_t count_bytes);
#define BLIBC_ARENA_MAKE_SLICE(Arena,Type, count) ((Type){.data =blibc_arena_alloc(Arena,sizeof(((Type*)(64))->data[0])*count), .len = count});


#define BLIBC_MAKE_VEC(Arena, Type) (Type){.arena = Arena, .data = 0, .len = 0, .cap = 0} 

#define BLIBC_VEC_PUSH(Vec, Item) {if ((Vec).len<(Vec).cap){\
    (Vec).data[(Vec).len] = Item; \
    (Vec).len += 1;\
} else{\
    if((Vec).cap<1){\
        (Vec).data = blibc_arena_alloc((Vec).arena, sizeof(*(Vec).data)*8); (Vec).cap = 8;\
    }else{\
        (Vec).data = blibc_arena_realloc((Vec).arena, (Vec).data, sizeof(*(Vec).data)*(Vec).cap, sizeof((Vec).data)*(Vec).cap*2); (Vec).cap*= 2;\
    }\
    (Vec).data[(Vec).len] = Item; \
    (Vec).len += 1;\
}}\

#define BLIBC_VEC_POP(Vec) (Vec).len-=1

#define BLIBC_VEC_INSERT(Vec, Item, At) {\
    if((Vec).len>0){\
        BLIBC_VEC_PUSH((Vec),(Vec).data[(Vec).len-1]);\
        memmove(&(Vec).data[At+1], &(Vec).data[At], sizeof(*(Vec).data)*((Vec).len-At));\
        (Vec).data[At] = Item;\
    }else if (At == 0){\
       BLIBC_VEC_PUSH(Vec, Item);\
    }else{\
        assert(false);\
    }\
}


#define BLIBC_VEC_REMOVE(Vec, At){\
    if((Vec).len>At){\
        memmove(&(Vec).data[At], &(Vec).data[At+1], sizeof(*(Vec).data)*((Vec).len-At));\
        (Vec).len-=1;\
    }\
}\

#ifdef USING_BLIBC
#define improve_type BLIBC_IMPROVE_TYPE
#define slice_get BLIBC_GET
#define make_slice BLIBC_MAKE_SLICE 
#define Arena BLIBC_Arena
#define arena_create blibc_arena_create
#define arena_destroy blibc_arena_destroy
#define arena_alloc blibc_arena_alloc
#define arena_realloc blibc arena_realloc 
#define arena_make_slice BLIBC_ARENA_MAKE_SLICE
#define make_vec BLIBC_MAKE_VEC 
#define vec_push BLIBC_VEC_PUSH
#define vec_pop BLIBC_VEC_POP 
#define vec_insert BLIBC_VEC_INSERT
#define vec_remove BLIBC_VEC_REMOVE 
#endif

#endif
