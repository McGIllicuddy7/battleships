#include "battleship.h"
#include <stdio.h>
#include <tgmath.h>
#include <stdarg.h>
void gameloop(void){
    bool has_quit = false;
    while (!has_quit){ 
        printf("press any key to play or q to quit:");
        int c = fgetc(stdin);
        if(c == 'q'){
            has_quit = true;
            break;
        }
        battleship_game_t * game = calloc(1, sizeof(battleship_game_t));
        run_game(game);
        free(game);
    }
}
void run_game(battleship_game_t * state){
    player_setup_entities(state);
    ai_setup_entities(state);
    while(true){
        bool player_has_ships = false;
        bool ai_has_ships = false;
        for(size_t i =0; i<BATTLESHIP_MAX_ENTITY_COUNT; i++){
            if (state->entities[i].is_valid && state->entities[i].kind == TYPE_SHIP){
                if (state->entities[i].allegience == PLAYER_ALLEGIENCE){
                    player_has_ships = true;
                }
                if(state->entities[i].allegience == AI_ALLEGIENCE){
                    ai_has_ships = true;
                }
            }
        }
        if(!player_has_ships && !ai_has_ships){
            printf("game over: draw\n");
            return;
        }else if(player_has_ships && !ai_has_ships){
            printf("game over: you won!\n");
            return;
        }else if(!player_has_ships && ai_has_ships){
            printf("game over: you lost!\n");
            return;
        }
        game_update(state);
    }
}

void game_update(battleship_game_t * state){
    blibc_arena_t * arena = blibc_arena_create();
    battleship_move_vec_t player_moves = player_calculate_actions(arena, state);
    battleship_move_vec_t ai_moves = ai_calculate_actions(arena, state);
    for(size_t i =0; i<player_moves.len; i++){
        battleship_execute_move(state,player_moves.items[i]);
    }
    for(size_t i =0; i<ai_moves.len; i++){
        battleship_execute_move(state,ai_moves.items[i]);
    }
    battleship_state_update(state);
    blibc_arena_destroy(arena);
}

void player_setup_entities(battleship_game_t * state){

}

void ai_setup_entities(battleship_game_t * state){

}


battleship_entity_vec_t get_active_entities(blibc_arena_t *arena,battleship_game_t * game, entity_ref_t querier){
    battleship_entity_vec_t out = BLIBC_MAKE_VEC(arena, battleship_entity_vec_t);
    battleship_entity_t * observer = get_entity(game, querier);
    if(!observer){
        return out;
    }
    for(size_t i =0; i<BATTLESHIP_MAX_ENTITY_COUNT; i++){
        if(game->entities[i].is_valid){
            battleship_entity_t et = get_observered_state(&game->entities[i], observer->data.x, observer->data.y, observer->data.z);
            BLIBC_VEC_PUSH(out, et);
        }
    } 
    return out;
}

bool get_entity_rel(battleship_game_t * game, entity_ref_t querier, entity_ref_t to_get, battleship_entity_t * out){
    battleship_entity_t * observer = get_entity(game, querier);
    if(!observer){
        return false;
    }
    battleship_entity_t * output = get_entity(game, to_get);
    if(!output){
        return false;
    }
    *out = get_observered_state(output, observer->data.x, observer->data.y, observer->data.z);
    return true;
}

battleship_entity_t * get_entity(battleship_game_t* game, entity_ref_t ent){
    if(ent.idx>= BATTLESHIP_MAX_ENTITY_COUNT){
        return NULL;
    }
    battleship_entity_t* et = &game->entities[ent.idx];
    if(!et->is_valid|| et->generation != ent.generation){
        return NULL;
    }
    return et;
}

battleship_entity_t get_observered_state(battleship_entity_t* entity, i64 from_x, i64 from_y, i64 from_z){
    size_t best_guess = PREVIOUS_STATE_STORED_COUNT;
    i64 actual_dx = entity->data.x-from_x;
    i64 actual_dy = entity->data.y-from_y;
    i64 actual_dz = entity->data.z-from_z;
    i64 distance = sqrt((actual_dx*actual_dx)+(actual_dy*actual_dy)+(actual_dz*actual_dz));
    i64 delta_time =  (distance/(SPEED_OF_LIGHT_CM));
    for(size_t i =0 ;i<PREVIOUS_STATE_STORED_COUNT; i++){
        i64 dx = entity->relativity_previous_states[i].x-from_x;
        i64 dy = entity->relativity_previous_states[i].y-from_y;
        i64 dz = entity->relativity_previous_states[i].z-from_z;
        i64 distance = sqrt((dx*dx)+(dy*dy)+(dz*dz));
        i64 dt = (i*BATTLESHIP_GAME_TURN_TIME_MS*1000 - distance/SPEED_OF_LIGHT_CM);
        if(llabs(dt)< llabs(delta_time)){
            best_guess = i;
            delta_time = dt;
        }
    }
    battleship_visible_data_t data;
    if(best_guess == PREVIOUS_STATE_STORED_COUNT){
        data = entity->data;
    }else{
        data = entity->relativity_previous_states[best_guess];
    }
    battleship_entity_t out = *entity;
    out.data = data;
    return out;
}
battleship_move_opt_t parse_player_command(blibc_arena_t * arena,battleship_game_t *game, entity_ref_t target,blibc_str_t str){
    battleship_entity_t* et_ptr = get_entity(game, target);
    if(!et_ptr){
        goto done;
    }
    battleship_entity_t current_entity = *et_ptr;
    blibc_str_vec_t cmd = blibc_str_split_whitespace(arena,str);
    if(cmd.len<1){
        goto error;
    } 
    if(blibc_str_eq(cmd.items[0], BLIBC_STR("move"))){
        if(cmd.len<4){
            goto error;
        }
        if(!blibc_str_is_integer(cmd.items[1])){
            goto error;
        }
        if(!blibc_str_is_integer(cmd.items[2])){
            goto error;
        }
        if(!blibc_str_is_integer(cmd.items[3])){
            goto error;
        }
        i64 x = atoll(blibc_str_to_c_string(arena,cmd.items[1]));
        i64 y = atoll(blibc_str_to_c_string(arena,cmd.items[2]));
        i64 z = atoll(blibc_str_to_c_string(arena,cmd.items[3]));
        battleship_move_t move;
        move.kind = MOVE_MOVE_TOWARD;
        move.source = target;
        move.target_x = x;
        move.target_y = y;
        move.target_z = z;
        return (battleship_move_opt_t){.is_valid = true, .value = move};
    }else if(blibc_str_eq(cmd.items[0], BLIBC_STR("help"))){
        battleship_printf("commands:\n");
        battleship_printf("to move towards a point:move {x} {y} {z}\n");
        battleship_printf("to fire a missile at a target: fire missile at {name of target}\n");
        battleship_printf("to fire laser at a target: fire laser at {name of target}\n");
        battleship_printf("to show game state(relative to current ship): show\n");
        battleship_printf("to show this message: help\n");
        goto done;
    }else if(blibc_str_eq(cmd.items[0], BLIBC_STR("fire"))){
        if(cmd.len<4){
            goto error;
        }
        blibc_str_t weapon = cmd.items[1];
        battleship_move_t move;
        if(blibc_str_eq(weapon, BLIBC_STR("missile"))){
            move.kind = MOVE_FIRE_MISSILE;
        }else if(blibc_str_eq(weapon, BLIBC_STR("laser"))){
            move.kind = MOVE_FIRE_LASER;
        }else{
            goto error;
        }
        if(!blibc_str_eq(cmd.items[2] ,BLIBC_STR("at"))){
            goto error;
        }
        i32 target_idx = -1;
        for(i32 i =0; i< BATTLESHIP_MAX_ENTITY_COUNT; i++){
            battleship_entity_t * ent = &game->entities[i];
            if(!ent->is_valid){
                continue;
            }
            if(blibc_str_eq(ent->name, cmd.items[3])){
                target_idx = i;
                break;
            }
        }
        if(target_idx == -1){
            goto error;
        }
        battleship_entity_t* out_target = &game->entities[target_idx];
        if(!out_target->is_valid){
           goto error; 
        }
        entity_ref_t target_ref = (entity_ref_t){.generation = out_target->generation, .idx = target_idx};
        if(target_idx == target.idx){
            battleship_printf("ship cannot target itself\n");
            goto error;
        }
    }else if(blibc_str_eq(cmd.items[0], BLIBC_STR("show"))){
        for(size_t i =0; i<BATTLESHIP_MAX_ENTITY_COUNT; i++){
            battleship_entity_t* ent = &game->entities[i];
            if(!ent->is_valid){
                continue;
            }
            battleship_entity_t ob_ent = get_observered_state(ent, current_entity.data.x, current_entity.data.y, current_entity.data.z);
            if(ent->kind == TYPE_SHIP || ent->kind == TYPE_INANIMATE){
                battleship_printf(BLIBC_STR_FMT"\n", BLIBC_STR_ARG(ent->name));
                battleship_printf("   velocity: x:%lld y:%lld z:%lld\n", ob_ent.data.velocity_x, ob_ent.data.velocity_y, ob_ent.data.velocity_z);
                battleship_printf("   position: x:%lld y:%lld z:%lld\n", ob_ent.data.x, ob_ent.data.y, ob_ent.data.z);
            }
        }
        goto done;
    }else{
        goto error;
    }
error:
    battleship_printf("error unknown command, enter \"help\" to get a list of acceptable commands\n");
done:
    return (battleship_move_opt_t){0};
}

battleship_move_vec_t player_calculate_actions(blibc_arena_t * arena,battleship_game_t * game){
    battleship_move_vec_t out = BLIBC_MAKE_VEC(arena, battleship_move_vec_t);
    for(size_t i =0; i<BATTLESHIP_MAX_ENTITY_COUNT; i++){
        if(game->entities[i].is_valid && game->entities[i].allegience == PLAYER_ALLEGIENCE){
            entity_ref_t et;
            et.generation = game->entities[i].generation;
            et.idx = i;
            battleship_printf("enter move for "BLIBC_STR_FMT": ", BLIBC_STR_ARG(game->entities[i].name));
            while(true){
                blibc_str_t str = battleship_get_line(arena);
                battleship_move_opt_t move_opt = parse_player_command(arena, game, et,str);
                if(move_opt.is_valid){
                    BLIBC_VEC_PUSH(out, move_opt.value);
                    break;
                }
            }
        }
    }
    return out;
}

battleship_move_vec_t ai_calculate_actions(blibc_arena_t * arena,battleship_game_t * game){
    battleship_move_vec_t out = BLIBC_MAKE_VEC(arena, battleship_move_vec_t);
    for(size_t i = 0; i<BATTLESHIP_MAX_ENTITY_COUNT; i++){
            if(game->entities[i].is_valid && game->entities[i].allegience == AI_ALLEGIENCE){
                entity_ref_t ship;
                ship.idx = i;
                ship.generation = game->entities[i].generation;
                battleship_move_t move = get_ai_ship_move(game, ship);
                BLIBC_VEC_PUSH(out, move);
            } 
    }
    return out;
}

blibc_str_t battleship_get_line(blibc_arena_t * arena){
    char buffer [1024] = {0};
    fgets(buffer, 1024, stdin);
    size_t len = strlen(buffer);
    char * ptr = blibc_arena_alloc(arena, len);
    blibc_str_t out;
    out.items = ptr;
    out.len = len;
    return out;
}

i32 battleship_printf(const char * ptr, ...){
    va_list list;
    va_start(list, ptr);
    i32 out = vfprintf(stdout,ptr, list);
    va_end(list);
    return out;
}

battleship_move_t get_ai_ship_move(
    battleship_game_t * game, entity_ref_t ship
){
    battleship_move_t out = {0};
    out.kind = MOVE_MOVE_TOWARD;
    out.source = ship;
    out.target_x = (rand()%1000-500)*100;
    out.target_y = (rand()%1000-500)*100;
    out.target_y = (rand()%1000-500)*100;
    return out;
}

void battleship_execute_move(battleship_game_t * game, battleship_move_t move){

}
void battleship_state_update(battleship_game_t *state){

}