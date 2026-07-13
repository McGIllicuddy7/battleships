pub use crate::utils::{Fx64, Vec3};
use std::sync::Arc;
#[derive(Debug, Clone, Copy)]
pub struct EntityVisibleData {
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

pub const MAX_ENTITY_COUNT: usize = 65536;
#[derive(Clone, Debug)]
pub struct GameState {
    pub entities: Box<[Entity]>,
}
