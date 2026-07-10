#ifndef BRIDGET_LIBC_GAME_UTILS_H
#define BRIDGET_LIBC_GAME_UTILS_H
/*
#include <raylib.h>
#include "blibc.h"
typedef struct {
    u32 idx;
    u32 generation;
}entity_id_t;

typedef struct {
    entity_id_t id;
    Vector3 pos;
    Vector3 velocity;
    Vector3 angular_velocity;
    Quaternion rotation;
    float width;//x
    float depth;//y
    float height;//z    
}entity_data_3d_t;
BLIBC_IMPROVE_TYPE(entity_data_3d_t, entity_data_3d);


typedef struct {
    entity_id_t id;
    Vector2 pos;
    Vector2 velocity;
    float angular_velocity;
    float rotation;
    float width;
    float height;
}entity_data_2d_t;

BLIBC_IMPROVE_TYPE(entity_data_2d_t, entity_data_2d);


typedef struct {
    entity_id_t hit_id;
    Vector3 normal;
    Vector3 relative_velocity_hit_entity_stationary;
}collision_event_3d_t;

BLIBC_IMPROVE_TYPE(collision_event_3d_t, collision_event_3d);

typedef struct {
    entity_id_t hit_id;
    Vector2 normal;
    Vector2 relative_velocity_hit_entity_stationary;
}collision_event_2d_t;
BLIBC_IMPROVE_TYPE(collision_event_2d_t, collision_event_2d);

typedef struct {
    u32 to_id;
    f32 distance;
}graph_connection_t;

BLIBC_IMPROVE_TYPE(graph_connection_t, graph_connection);

typedef struct {
    u32 self_id;
    graph_connection_vec_t connections; 
}graph_data_t;
BLIBC_IMPROVE_TYPE(graph_data_t, graph_data);

bool check_collision_entities_3d(const entity_data_3d_t * a, const entity_data_3d_t *b, Vector3 * normal);
collision_event_3d_vec_t blibc_update_entities_3d(entity_data_3d_t ** entities, size_t entities_count);

collision_event_2d_vec_t blibc_update_entities_2d(entity_data_2d_t ** entities, size_t entities_count);
*/
#endif