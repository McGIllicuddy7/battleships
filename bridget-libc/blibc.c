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


typedef union {
	u8 bytes[1];
	u8 value;
}u8_byte_union_t;

typedef union{
	u8 bytes[2];
	u16 value;
}u16_byte_union_t;

typedef union{
	u8 bytes[4];
	u32 value;
}u32_byte_union_t;

typedef union{
	u8 bytes[8];
	u64 value;
}u64_byte_union_t;

typedef union {
	u8 bytes[1];
	i8 value;
}i8_byte_union_t;
typedef union{
	u8 bytes[2];
	i16 value;
}i16_byte_union_t;

typedef union{
	u8 bytes[4];
	i32 value;
}i32_byte_union_t;

typedef union{
	u8 bytes[8];
	i64 value;
}i64_byte_union_t;

typedef union {
	u8 bytes[4];
	f32 value;
}f32_byte_union_t;

typedef union{
	u8 bytes[8];
	f64 value;
}f64_byte_union_t;

void blibc_handle_endianness(u8 * byte_buffer, size_t count){
	if(count <= 1){
		return;
	}
	for(size_t i =0; i<count/2; i++){
		u8 tmp = byte_buffer[i];
		u8 tmp2 = byte_buffer[count-i];
		byte_buffer[i] = tmp;
		byte_buffer[count-i] = tmp2;
	}
}

u8_opt_t blibc_deserialize_u8_no_tag(blibc_deserializer_t * des){
	if(des->ptr<des->data.len){
		u8 value = des->data.items[des->ptr];
		des->ptr++;
		return (u8_opt_t){.value =value, .is_valid = true };
	}else{
		return (u8_opt_t){.value =0, .is_valid = false};
	}
}

void blibc_serialize_u8_no_tag(blibc_serializer_t * ser, u8 value){
	u8_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_bytes(blibc_serializer_t * ser,u8 * start, size_t count){
	blibc_serialize_tag(ser, blibc_serial_data_tag_byte_list);
	for(size_t i =0; i<count; i++){
		BLIBC_VEC_PUSH(ser->data, start[i]);
	}  
}


void blibc_serialize_u8(blibc_serializer_t * ser, u8 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_u8);
	u8_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_u16(blibc_serializer_t * ser, u16 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_u16);
	u16_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}
void blibc_serialize_u32(blibc_serializer_t * ser, u32 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_u32);
	u32_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_u64(blibc_serializer_t * ser, u64 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_u64);
	u64_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_i8(blibc_serializer_t * ser, i8 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_i8);
	i8_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_i16(blibc_serializer_t * ser, i16 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_i16);
	i16_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_i32(blibc_serializer_t * ser, i32 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_i32);
	i32_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_i64(blibc_serializer_t * ser, i64 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_i64);
	i64_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){	
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_f32(blibc_serializer_t * ser, f32 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_f32);
	f32_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_f64(blibc_serializer_t * ser, f64 value){
	blibc_serialize_tag(ser, blibc_serial_data_tag_f64);
	f64_byte_union_t bytes;
	bytes.value = value;
	blibc_handle_endianness(bytes.bytes, sizeof(bytes));
	for(size_t i =0; i<sizeof(bytes.bytes); i++){
		BLIBC_VEC_PUSH(ser->data, bytes.bytes[i]);
	}
}

void blibc_serialize_bool(blibc_serializer_t * ser, bool b){
	blibc_serialize_tag(ser, blibc_serial_data_tag_bool);
	if(b){
		blibc_serialize_u8(ser,1);
	}else{
		blibc_serialize_u8(ser,0);
	}
}

void blibc_serialize_str(blibc_serializer_t * ser, blibc_str_t str){
	blibc_serialize_tag(ser, blibc_serial_data_tag_str);
	blibc_serialize_u64(ser, (u64)str.len);
	blibc_serialize_bytes(ser,(u8*)str.items, (u64)str.len);
}

bool blibc_deserialize_bytes(blibc_deserializer_t * des, u8 * output, size_t count){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_byte_list);
	if(!tag_opt.is_valid){
		return false;
	}
	if(!tag_opt.value){
		return false;
	}
	for(size_t i =0; i<count; i++){
		u8_opt_t tmp = blibc_deserialize_u8(des);
		if(!tmp.is_valid){
			return false;
		}
		output[i] = tmp.value;
	} 
	return true;
}


u8_opt_t blibc_deserialize_u8(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_u8);
	if(!tag_opt.is_valid){
		return (u8_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (u8_opt_t){.value = 0., .is_valid = false};
	}
	u8_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (u8_opt_t){.is_valid = false, .value =0};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (u8_opt_t){.is_valid = true, .value = buf.value};
}

u16_opt_t blibc_deserialize_u16(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_u16);
	if(!tag_opt.is_valid){
		return (u16_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (u16_opt_t){.value = 0., .is_valid = false};
	}
	u16_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);	
		if(!tmp.is_valid){	
			return (u16_opt_t){.is_valid = false, .value =0};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (u16_opt_t){.is_valid = true, .value = buf.value};
}

u32_opt_t blibc_deserialize_u32(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_u32);
	if(!tag_opt.is_valid){
		return (u32_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (u32_opt_t){.value = 0., .is_valid = false};
	}
	u32_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (u32_opt_t){.is_valid = false, .value =0};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (u32_opt_t){.is_valid = true, .value = buf.value};
}

u64_opt_t blibc_deserialize_u64(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_u64);
	if(!tag_opt.is_valid){
		return (u64_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (u64_opt_t){.value = 0., .is_valid = false};
	}
	u64_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (u64_opt_t){.is_valid = false, .value =0};
		}
	        buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (u64_opt_t){.is_valid = true, .value = buf.value};
}

i8_opt_t blibc_deserialize_i8(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_i8);
	if(!tag_opt.is_valid){
		return (i8_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (i8_opt_t){.value = 0., .is_valid = false};
	}

	i8_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
	u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
	if(!tmp.is_valid){
		return (i8_opt_t){.is_valid = false, .value =0};
	}
	buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (i8_opt_t){.is_valid = true, .value = buf.value};
}

i16_opt_t blibc_deserialize_i16(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_i16);
	if(!tag_opt.is_valid){
		return (i16_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (i16_opt_t){.value = 0., .is_valid = false};
	}
	i16_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (i16_opt_t){.is_valid = false, .value =0};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return
     (i16_opt_t){.is_valid = true, .value = buf.value};
}

i32_opt_t blibc_deserialize_i32(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_i32);
	if(!tag_opt.is_valid){
		return (i32_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (i32_opt_t){.value = 0., .is_valid = false};
	}

	i32_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (i32_opt_t){.is_valid = false, .value =0};
        	}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (i32_opt_t){.is_valid = true, .value = buf.value};
}

i64_opt_t blibc_deserialize_i64(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_i64);
	if(!tag_opt.is_valid){
		return (i64_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (i64_opt_t){.value = 0., .is_valid = false};
	}
	i64_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
        	u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
			return (i64_opt_t){.is_valid = false, .value =0};
        	}
        	buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (i64_opt_t){.is_valid = true, .value = buf.value};
}

f32_opt_t blibc_deserialize_f32(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_f32);
	if(!tag_opt.is_valid){
		return (f32_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (f32_opt_t){.value = 0., .is_valid = false};
	}
	f32_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
		return (f32_opt_t){.is_valid = false, .value =0.};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (f32_opt_t){.is_valid = true, .value = buf.value};
}

f64_opt_t blibc_deserialize_f64(blibc_deserializer_t *des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_f64);
	if(!tag_opt.is_valid){
		return (f64_opt_t){.value = 0., .is_valid = false};
	}
	if(!tag_opt.value){
		return (f64_opt_t){.value = 0., .is_valid = false};
	}
	f64_byte_union_t buf;
	for(size_t i =0; i<sizeof(buf.bytes); i++){
		u8_opt_t tmp = blibc_deserialize_u8_no_tag(des);
		if(!tmp.is_valid){
				return (f64_opt_t){.is_valid = false, .value =0.};
		}
		buf.bytes[i] = tmp.value;
	}
	blibc_handle_endianness(buf.bytes, sizeof(buf));
	return (f64_opt_t){.is_valid = true, .value = buf.value};
}

bool_opt_t blibc_deserialize_bool(blibc_deserializer_t * des){
	bool_opt_t tag_opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_bool);
	if(!tag_opt.is_valid){
		return (bool_opt_t){.value = false, .is_valid = false};
	}
	if(!tag_opt.value){
		return (bool_opt_t){.value = false, .is_valid = false};
	}
	u8_opt_t opt = blibc_deserialize_u8_no_tag(des);
	if(!opt.is_valid){
		return (bool_opt_t){.is_valid = false, .value = false};
	}
	if(opt.value == 1){
		return (bool_opt_t){.is_valid = true, .value = true};
	}else if(opt.value == 0){
        	return (bool_opt_t){.is_valid = true, .value = false};
	}else{
        	return (bool_opt_t){.is_valid = false, .value = false};
	}
}

blibc_str_t blibc_deserialize_str(blibc_deserializer_t * des){
	bool_opt_t opt = blibc_deserialize_check_tag(des, blibc_serial_data_tag_str);
	if(!opt.is_valid){
		return (blibc_str_t){.items =0, .len =0};
	}
	if(!opt.value){
		return (blibc_str_t){.items =0, .len =0};
	}
	u64_opt_t size_value = blibc_deserialize_u64(des);
	if (!size_value.is_valid){
		return (blibc_str_t){0};
	}
	char * bytes = blibc_arena_alloc(des->allocator
,size_value.value);
	if(!blibc_deserialize_bytes(des, (u8*)bytes, size_value.value)){
		return (blibc_str_t){0};
	}
	blibc_str_t out;
	out.items = bytes;
	out.len = size_value.value;
	return out;
}

void blibc_serialize_tag(blibc_serializer_t *ser, blibc_serial_data_tag_t tag){
	blibc_serialize_u8_no_tag(ser, tag);
}

bool_opt_t blibc_deserialize_check_tag(blibc_deserializer_t * des, blibc_serial_data_tag_t tag){
	u8_opt_t opt = blibc_deserialize_u8_no_tag(des);
	if(!opt.is_valid){
		return (bool_opt_t){.value = false, .is_valid = false};
	}
	return (bool_opt_t){.is_valid = true, .value = (opt.value == tag)};
}


bool blibc_str_eq(blibc_str_t a, blibc_str_t b){
	if(a.len != b.len){
		return false;
	}
	for(size_t i =0; i<a.len; i++){
		if(a.items[i] != b.items[i]){
			return false;
		}
	}
	return true;
}

bool blibc_str_is_integer(blibc_str_t to_check){
	if(to_check.len<1){
		return false;
	}
	if(!(isnumber(to_check.items[0]) || to_check.items[0] == '-')){
		return false;
	}
	if(to_check.items[0] == '-' && to_check.len<2){
		return false;
	}
	for(size_t i = 1; i<to_check.len; i++){
		if (!isnumber(to_check.items[i])){
			return false;
		}
	}
	return true;
}
bool blibc_str_is_double(blibc_str_t to_check){
		if(to_check.len<1){
		return false;
	}
	if(!(isnumber(to_check.items[0]) || to_check.items[0] == '-')){
		return false;
	}
	if(to_check.items[0] == '-' && to_check.len<2){
		return false;
	}
	bool has_hit_dot;
	for(size_t i = 1; i<to_check.len; i++){
		if (!isnumber(to_check.items[i])&& has_hit_dot){
			return false;
		}
		if(to_check.items[i] == '.'){
			has_hit_dot = true;
		}
	}
	return true;
}