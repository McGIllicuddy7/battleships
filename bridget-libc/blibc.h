#ifndef B_LIBC_H
#define B_LIBC_H
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>
#include <string.h>
void * blibc_debug_alloc(size_t count, size_t size);
void blibc_debug_free(void * ptr);
#define mem_alloc blibc_debug_alloc 
#define mem_free blibc_debug_free 

#define BLIBC_IMPROVE_TYPE(Type, Name)\
    typedef struct {\
        Type value;\
        bool is_valid;\
    }Name##_opt_t;\
    \
    typedef struct{\
        Type * items;\
        size_t len;\
    }Name##_slice_t;\
    \
    typedef struct {\
        struct blibc_arena_t * arena;\
        Type * items;\
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

    
#define BLIBC_GET(list, at) (*((blibc_check_bounds(list.len, at)),&list.items[at]))

#define BLIBC_MAKE_SLICE(Type, count) ((Type){.items =mem_alloc(sizeof(((Type*)(64))->items[0]),count), .len = count});

struct blibc_arena_t;
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
typedef struct blibc_arena_t blibc_arena_t;
blibc_arena_t * blibc_arena_create(void);
blibc_arena_t * blibc_arena_sized(size_t page_count);
void blibc_arena_destroy(blibc_arena_t * arena);
void * blibc_arena_alloc(blibc_arena_t * arena, size_t count_bytes);
void * blibc_arena_realloc(blibc_arena_t * arena,void * ptr, size_t old_size, size_t count_bytes);
#define BLIBC_ARENA_MAKE_SLICE(Arena,Type, count) ((Type){.items =blibc_arena_alloc(Arena,sizeof(((Type*)(64))->items[0])*count), .len = count});


#define BLIBC_MAKE_VEC(Arena, Type) (Type){.arena = Arena, .items = 0, .len = 0, .cap = 0} 

#define BLIBC_VEC_PUSH(Vec, Item) {if ((Vec).len<(Vec).cap){\
    (Vec).items[(Vec).len] = Item; \
    (Vec).len += 1;\
} else{\
    if((Vec).cap<1){\
        (Vec).items = blibc_arena_alloc((Vec).arena, sizeof(*(Vec).items)*8); (Vec).cap = 8;\
    }else{\
        (Vec).items = blibc_arena_realloc((Vec).arena, (Vec).items, sizeof(*(Vec).items)*(Vec).cap, sizeof((Vec).items)*(Vec).cap*2); (Vec).cap*= 2;\
    }\
    (Vec).items[(Vec).len] = Item; \
    (Vec).len += 1;\
}}\

#define BLIBC_VEC_POP(Vec) (Vec).len-=1

#define BLIBC_VEC_INSERT(Vec, Item, At) {\
    if((Vec).len>0){\
        BLIBC_VEC_PUSH((Vec),(Vec).items[(Vec).len-1]);\
        memmove(&(Vec).items[At+1], &(Vec).items[At], sizeof(*(Vec).items)*((Vec).len-At));\
        (Vec).items[At] = Item;\
    }else if (At == 0){\
       BLIBC_VEC_PUSH(Vec, Item);\
    }else{\
        assert(false);\
    }\
}


#define BLIBC_VEC_REMOVE(Vec, At){\
    if((Vec).len>At){\
        memmove(&(Vec).items[At], &(Vec).items[At+1], sizeof(*(Vec).items)*((Vec).len-At));\
        (Vec).len-=1;\
    }\
}\

typedef struct{
    char * items;
    size_t len;
} blibc_str_t;

BLIBC_IMPROVE_TYPE(blibc_str_t, blibc_str)

#define BLIBC_STR(STR) ((blibc_str_t){.items = (STR), .len = strlen((STR))} )

blibc_str_t blibc_str_concat(blibc_arena_t * arena, blibc_str_t s1, blibc_str_t s2);
blibc_str_t blibc_str_push(blibc_arena_t * arena, blibc_str_t st, char c);
blibc_str_t blibc_str_fmt(blibc_arena_t * arena, const char * fmt, ...);

#define BLIBC_STR_FMT "%.*s"
#define BLIBC_STR_ARG(ST) (int)((ST).len), (ST).items

blibc_str_vec_t blibc_str_split_by(blibc_arena_t * arena, blibc_str_t str, char delim);
blibc_str_vec_t blibc_str_split_whitespace(blibc_arena_t * arena, blibc_str_t str);

char * blibc_str_to_c_string(blibc_arena_t * arena, blibc_str_t st);

blibc_str_t blibc_read_file_to_string(blibc_arena_t * arena, blibc_str_t filename);
int blibc_write_string_to_file(blibc_str_t filename, blibc_str_t str);

blibc_str_t blibc_str_trim(blibc_str_t st);

#ifdef USING_BLIBC
#define improve_type BLIBC_IMPROVE_TYPE
#define slice_get BLIBC_GET
#define make_slice BLIBC_MAKE_SLICE 
typedef blibc_arena_t arena_t;

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

typedef blibc_str_t str_t;
#define STR BLIBC_STR 
#define str_concat blibc_str_concat
#define str_push blibc_str_push 
#define str_fmt blibc_str_fmt
#define STR_FMT BLIBC_STR_FMT
#define STR_ARG BLIBC_STR_ARG
typedef blibc_str_opt_t str_opt;
typedef blibc_str_owned_t str_owned_t;
typedef blibc_str_rc_t str_rc_t;
typedef blibc_str_vec_t str_vec_t;

#define str_split_by blibc_str_split_by
#define str_split_whitespace blibc_str_split_whitespace

#define str_to_c_string blibc_str_to_c_string

#define read_file_to_string blibc_read_file_to_string
#define write_string_to_file blibc_write_string_to_file

#endif


#define for_each(Type,Item, Collection) Type Item = (Collection).items[0]; for (size_t Item##_idx1 = 0;  Item##_idx1< (Collection).len;  Item = (Collection).items[++Item##_idx1])

#define blibc_enable_hash_map(KeyType,ValueType,Name)\
typedef struct {\
    KeyType key;\
    ValueType value;\
} Name##_key_value_pair_t;\
typedef struct Name##_bucket_t{\
    Name##_key_value_pair_t pair;\
    struct Name##_bucket_t * next;\
}Name##_bucket_t;\
typedef struct {\
    Name##_bucket_t** buckets;\
    size_t bucket_len;\
    u64 (*key_hash_fn)(KeyType);\
    bool (*key_eq_fn)(KeyType, KeyType);\
    void (*key_destructor)(KeyType);\
    void (*value_destructor)(ValueType);\
}Name;\
__unused inline static size_t Name##_len(Name * self){\
    size_t out =0;\
    for(size_t i =0; i<self->bucket_len; i++){\
        Name##_bucket_t * current = self->buckets[i];\
        while(current){\
            out+=1;\
            current = current->next;\
        }\
    }\
    return out;\
}\
__unused inline static void Name##_resize(Name * self, size_t size){\
    Name##_bucket_t ** new_set = mem_alloc(sizeof(Name##_bucket_t*),size);\
    for(size_t i =0; i<self->bucket_len; i++){\
        Name##_bucket_t * current = self->buckets[i];\
        while(current){\
            Name##_bucket_t * tmp =current->next;\
            current->next = 0;\
            size_t hash = self->key_hash_fn(current->pair.key)%size;\
            Name##_bucket_t ** prev = &new_set[hash];\
            Name##_bucket_t * ns = new_set[hash];\
            while(ns){\
                prev = &ns->next;\
                ns = ns->next;\
            }\
            *prev = current;\
            current = tmp;\
        }\
    }\
    Name##_bucket_t ** old = self->buckets;\
    self->buckets = new_set;\
    self->bucket_len = size;\
    mem_free(old);\
}\
__unused inline static Name* Name##_create(    u64 (*key_hash_fn)(KeyType),bool (*key_eq_fn)(KeyType, KeyType),void (*key_destructor)(KeyType), void (*value_destructor)(ValueType), size_t bucket_count){\
    Name * out = mem_alloc(1,sizeof(Name));\
    out->buckets = mem_alloc(bucket_count,sizeof(Name##_bucket_t*));\
    out->bucket_len = bucket_count;\
    out->key_hash_fn = key_hash_fn;\
    out->key_eq_fn = key_eq_fn;\
    out->key_destructor = key_destructor;\
    out->value_destructor = value_destructor;\
    return out;\
}\
__unused inline static ValueType* Name##_get(Name* self, KeyType* key){\
    size_t hash = self->key_hash_fn(*key)%self->bucket_len;\
    Name##_bucket_t* current =  self->buckets[hash];\
    while(current){\
        if(self->key_eq_fn(current->pair.key, *key)){\
            return &current->pair.value;\
        }\
        current = current->next;\
    }\
    return 0;\
}\
__unused inline static void Name##_insert(Name * self, KeyType key, ValueType value){\
    if(Name##_len(self)/2>self->bucket_len){\
        Name##_resize(self, self->bucket_len*2);\
    }\
    size_t hash = self->key_hash_fn(key)%self->bucket_len;\
    Name##_bucket_t ** prev = &self->buckets[hash];\
    Name##_bucket_t * current = self->buckets[hash];\
    while(current){\
        if(self->key_eq_fn(current->pair.key, key)){\
            self->key_destructor(current->pair.key);\
            self->value_destructor(current->pair.value);\
            current->pair.value = value;\
            current->pair.key = key;\
            return;\
        }\
        prev = &current->next;\
        current = current->next;\
    }\
    Name##_bucket_t * tmp = mem_alloc(1,sizeof(Name##_bucket_t));\
    tmp->next = 0;\
    tmp->pair.key = key;\
    tmp->pair.value = value;\
    *prev = tmp;\
}\
__unused inline static void Name##_remove(Name * self, KeyType key){\
    size_t hash = self->key_hash_fn(key)%self->bucket_len;\
    Name##_bucket_t ** prev = &self->buckets[hash];\
    Name##_bucket_t * current = self->buckets[hash];\
    while(current){\
        if(self->key_eq_fn(current->pair.key, key)){\
            self->key_destructor(current->pair.key);\
            self->value_destructor(current->pair.value);\
            *prev = current->next;\
        }\
        prev = &current->next;\
        current = current->next;\
    }\
}\
__unused inline static void Name##_for_each(Name * self, void (*to_run)(KeyType, ValueType *, void *), void * data){\
    for(size_t i =0; i<self->bucket_len; i++){\
        Name##_bucket_t * current = self->buckets[i];\
        while(current){\
            to_run(current->pair.key, &current->pair.value, data);\
            current = current->next;\
        }\
    }\
}\
__unused inline static void Name##_destroy(Name * self){\
    for(size_t i =0; i<self->bucket_len; i++){\
        Name##_bucket_t * current = self->buckets[i];\
        while(current){\
            Name##_bucket_t * next = current->next;\
            self->key_destructor(current->pair.key);\
            self->value_destructor(current->pair.value);\
            mem_free(current);\
            current = next;\
        }\
    }\
    mem_free(self->buckets);\
    mem_free(self);\
}\
__unused inline static void Name##_clear(Name * self){\
       for(size_t i =0; i<self->bucket_len; i++){\
        Name##_bucket_t * current = self->buckets[i];\
        while(current){\
            Name##_bucket_t * next = current->next;\
            self->key_destructor(current->pair.key);\
            self->value_destructor(current->pair.value);\
            mem_free(current);\
            current = next;\
        }\
        self->buckets[i] =0;\
    }\ 
}\

void debug_alloc_free_counts(void);
void print_alloc_free_counts(void);
#endif
