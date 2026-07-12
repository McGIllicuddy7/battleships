#ifndef BATTLESHIP_H
#define BATTLESHIP_H
#include "../bridget-libc/blibc.h"

#define BATTLESHIP_GAME_TURN_TIME_MS 100

#define BATTLESHIP_MAX_ENTITY_COUNT 4096
#define PREVIOUS_STATE_STORED_COUNT 256
#define NEUTRAL_AlLEGIENCE 0
#define PLAYER_ALLEGIENCE 1
#define AI_ALLEGIENCE 2
#define SPEED_OF_LIGHT_CM 300000000000L

typedef struct {
    u32 idx;
    u32 generation;
}entity_ref_t;


BLIBC_IMPROVE_TYPE(entity_ref_t, entity_ref)

typedef enum{
    TYPE_INANIMATE,
    TYPE_MISSILE, 
    TYPE_SHIP
}battleship_entity_kind_t;


typedef struct {
    u32 health;
    i64 x;//centimeters
    i64 y;//centimeters
    i64 z;//centimeters
    i64 velocity_x;//centimeters per second
    i64 velocity_y;//centimeters per second
    i64 velocity_z;//centimeters per second
    u64 remaining_fuel;
    entity_ref_t target;
}battleship_visible_data_t;


typedef struct {
    bool is_valid;
    u32 generation;
    u8 allegience;
    blibc_str_t name;
    battleship_entity_kind_t kind;
    battleship_visible_data_t data;
    battleship_visible_data_t relativity_previous_states[PREVIOUS_STATE_STORED_COUNT];
}battleship_entity_t;
BLIBC_IMPROVE_TYPE(battleship_entity_t, battleship_entity)

typedef struct {
    battleship_entity_t entities[BATTLESHIP_MAX_ENTITY_COUNT];
    i64 sun_pos_x;
    i64 sun_pos_y;
    i64 sun_pos_z;
}battleship_game_t;

typedef enum {
    MOVE_FIRE_LASER, 
    MOVE_FIRE_MISSILE,
    MOVE_MOVE_TOWARD,
}battleship_move_type_t;

typedef struct {
    battleship_move_type_t kind;
    entity_ref_t source;
    i32 target_x;
    i32 target_y;
    i32 target_z;
    entity_ref_t target;
} battleship_move_t;
BLIBC_IMPROVE_TYPE(battleship_move_t, battleship_move)
void gameloop(void);
void player_setup_entities(battleship_game_t * state);
void ai_setup_entities(battleship_game_t * state);
void run_game(battleship_game_t * state);
void game_update(battleship_game_t * game);

battleship_entity_t * get_entity(battleship_game_t* game, entity_ref_t ent);
bool get_entity_rel(battleship_game_t * game, entity_ref_t querier, entity_ref_t to_get, battleship_entity_t * out);

battleship_entity_vec_t  get_active_entities(blibc_arena_t *arena,battleship_game_t * game, entity_ref_t querier);

battleship_entity_t get_observered_state(battleship_entity_t* entity, i64 from_x, i64 from_y, i64 from_z);

battleship_move_vec_t player_calculate_actions(blibc_arena_t * arena, battleship_game_t * game);


battleship_move_vec_t ai_calculate_actions(blibc_arena_t *arena,battleship_game_t * game);

battleship_move_opt_t parse_player_command(blibc_arena_t * arena,battleship_game_t *game, entity_ref_t target,blibc_str_t str);
blibc_str_t battleship_get_line(blibc_arena_t * arena);
i32 battleship_printf(const char * ptr, ...);

battleship_move_t get_ai_ship_move(
    battleship_game_t * game, entity_ref_t ship
);

void battleship_execute_move(battleship_game_t * game, battleship_move_t move);
void battleship_state_update(battleship_game_t *state);
#endif

