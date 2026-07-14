pub use crate::utils::{Fx, Vec3};
use std::sync::Arc;

pub const MAX_ENTITY_COUNT: usize = 65536;
pub const SPEED_OF_LIGHT: Fx = Fx::new(299_792_458);
pub const TURN_DELTA_TIME_SECS: Fx = Fx::new(1).op_div(&Fx::new(10));

#[derive(Debug, Clone, Copy)]
pub struct EntityVisibleData {
    pub existed: bool,
    pub position: Vec3,
    pub velocity: Vec3,
    pub health: i64,
}

#[derive(Clone, Copy, Debug)]
pub enum EntityKind {
    Inanimate,
    Ship,
    Missile,
    Laser,
}

#[derive(Clone, Copy, Debug)]
pub enum EntityAllegiance {
    Neutral = 0,
    Player = 1,
    Enemy = 2,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub is_valid: bool,
    pub generation: u64,
    pub kind: EntityKind,
    pub allegiance: EntityAllegiance,
    pub name: Arc<str>,
    pub current_state: EntityVisibleData,
    pub previous_states: [EntityVisibleData; 256],
}

#[derive(Clone, Copy, Debug)]
pub struct EntityRef {
    pub idx: usize,
    pub genr: u64,
}
#[derive(Clone, Debug)]
pub struct GameState {
    pub entities: Box<[Entity]>,
    pub entity_write_buffer: Box<[Entity]>,
}

impl GameState {
    pub fn new() -> Self {
        let mut entities = Vec::new();
        let mut entity_writes = Vec::new();
        entities.reserve_exact(MAX_ENTITY_COUNT);
        entity_writes.reserve_exact(MAX_ENTITY_COUNT);
        for _ in 0..MAX_ENTITY_COUNT {
            entities.push(Entity::new());
            entity_writes.push(Entity::new());
        }
        Self {
            entities: entities.into_boxed_slice(),
            entity_write_buffer: entity_writes.into_boxed_slice(),
        }
    }

    pub fn get_entity(&self, rf: EntityRef) -> Option<&Entity> {
        let gt = self.entities.get(rf.idx)?;
        if !gt.is_valid || gt.generation != rf.genr {
            return None;
        }
        Some(gt)
    }

    pub fn get_observed_entity(&self, rf: EntityRef, position: Vec3) -> Option<Entity> {
        let et = self.get_entity(rf)?;
        let dist = position.distance(et.current_state.position);
        let mut min_dt = dist / SPEED_OF_LIGHT;
        let mut min_idx = et.previous_states.len();
        for i in 0..et.previous_states.len() {
            let tdist = position.distance(et.previous_states[i].position);
            let dur = Fx::new(i as i64) * TURN_DELTA_TIME_SECS;
            let tdt = tdist / SPEED_OF_LIGHT - dur;
            let dt = if tdt < Fx::new(0) {
                tdt * (-1).into()
            } else {
                tdt
            };
            if dt < min_dt {
                min_dt = dt;
                min_idx = i;
            }
        }
        if min_idx >= et.previous_states.len() {
            Some(et.clone())
        } else {
            let mut et2 = et.clone();
            et2.current_state = et.previous_states[min_idx].clone();
            et2.previous_states = [EntityVisibleData::new(); _];
            Some(et2)
        }
    }

    pub fn get_writable_entity(&mut self, rf: EntityRef) -> Option<&mut Entity> {
        let g = self.entities.get_mut(rf.idx)?;
        if (!g.is_valid) || (g.generation != rf.genr) {
            None
        } else {
            Some(g)
        }
    }
}

impl Entity {
    pub fn new() -> Self {
        Self {
            is_valid: false,
            generation: 0,
            kind: EntityKind::Inanimate,
            allegiance: EntityAllegiance::Neutral,
            name: "".into(),
            current_state: EntityVisibleData::new(),
            previous_states: [EntityVisibleData::new(); _],
        }
    }
}

impl EntityVisibleData {
    pub const fn new() -> Self {
        Self {
            existed: false,
            position: Vec3::new_i64(0, 0, 0),
            velocity: Vec3::new_i64(0, 0, 0),
            health: 0,
        }
    }
}
