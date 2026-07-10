#include "battleship.h"
#include <stdio.h>
#include <tgmath.h>
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
        if(!blibc_str_is_integer(cmd.items[3])){
            goto error;
        }
        size_t target_idx = atoll(blibc_str_to_c_string(arena, cmd.items[3]));
        if(target_idx >= BATTLESHIP_MAX_ENTITY_COUNT){
            goto error;
        }
        battleship_entity_t* out_target = &game->entities[target_idx];
        if(!out_target->is_valid){
           goto error; 
        }
        entity_ref_t target_ref = (entity_ref_t){.generation = out_target->generation, .idx = target_idx};
        if(target_idx == target.idx){
            printf("ship cannot target itself\n");
            goto error;
        }
    }else if(blibc_str_eq(cmd.items[0], BLIBC_STR("show"))){
        goto done;
    }else{
        goto error;
    }
error:
    printf("error unknown command, enter \"help\" to get a list of acceptable commands\n");
done:
    return (battleship_move_opt_t){0};
}
battleship_move_vec_t player_calculate_actions(blibc_arena_t * arena,battleship_game_t * game){
    battleship_move_vec_t out = BLIBC_MAKE_VEC(arena, battleship_move_vec_t);

    return out;
}

battleship_move_vec_t ai_calculate_actions(blibc_arena_t * arena,battleship_game_t * game){
    battleship_move_vec_t out = BLIBC_MAKE_VEC(arena, battleship_move_vec_t);
    return out;
}