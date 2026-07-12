use std::sync::Arc;

#[derive(Clone, Debug, Copy)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityVisibleData {
    pub position: Vector,
    pub velocity: Vector,
    pub health: i64,
}

#[derive(Clone, Copy, Debug)]
pub enum EntityKind {
    Inanimate,
    Ship,
    Missile,
}

#[derive(Clone, Copy, Debug)]
pub enum EntityAllegiance {
    Neutral = 0,
    Player = 1,
    Enemy = 2,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub kind: EntityKind,
    pub allegiance: EntityAllegiance,
    pub name: Arc<str>,
    pub current_state: EntityVisibleData,
    pub previous_states: [EntityVisibleData; 256],
}
