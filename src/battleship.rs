use serde::{Deserialize, Serialize};

pub use crate::utils::{Fx, Vec3};
use crate::{gui::gets, ship_components::ComponentData};
use std::{collections::HashMap, sync::Arc};

pub const MAX_ENTITY_COUNT: usize = 65536;
pub const SPEED_OF_LIGHT: Fx = Fx::new(299_792_458);
pub const TURN_DELTA_TIME_SECS: Fx = Fx::new(1).op_div(&Fx::new(10));

#[derive(Debug, Clone, Copy)]
pub struct EntityVisibleData {
    pub existed: bool,
    pub position: Vec3,
    pub velocity: Vec3,
    pub forward_vector: Vec3,
    pub up_vector: Vec3,
    pub health: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityKind {
    Inanimate,
    Ship,
    Missile,
    Laser,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityAllegiance {
    Neutral = 0,
    Player1 = 1,
    Player2 = 2,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub is_valid: bool,
    pub generation: u64,
    pub kind: EntityKind,
    pub allegiance: EntityAllegiance,
    pub name: Arc<str>,
    pub width: Fx,
    pub height: Fx,
    pub depth: Fx,
    pub current_state: EntityVisibleData,
    pub previous_states: [EntityVisibleData; 256],
    pub components: ComponentData,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct EntityRef {
    pub idx: usize,
    pub genr: u64,
}
#[derive(Clone, Debug)]
pub struct GameState {
    pub entities: Box<[Entity]>,
    pub entity_write_buffer: Box<[Entity]>,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum WeaponKind {
    RailGun,
    PulseXRayLaser,
    PulseGammaRayLaser,
    PulseGreenLaser,
    GatlingGun,
    NuclearTorpedo,
    ConventionalTorpedo,
    BurningLaser,
    Chaff,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameCmd {
    SetCourseFor {
        entity: EntityRef,
        to: Vec3,
    },
    SetCourseForEntity {
        entity: EntityRef,
        target: EntityRef,
    },
    PursueEntity {
        entity: EntityRef,
        target: EntityRef,
    },
    Accelerate {
        entity: EntityRef,
        direction: Vec3,
    },
    FireWeapon {
        entity: EntityRef,
        at: EntityRef,
        weapon_kind: WeaponKind,
    },
    FireWeaponAtPoint {
        entity: EntityRef,
        at: Vec3,
    },
    Repair {
        entity: EntityRef,
        component: Arc<str>,
    },
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
            if !et.previous_states[min_idx].existed {
                return None;
            }
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

    pub fn get_all_entities(&self) -> Vec<EntityRef> {
        let mut out = Vec::new();
        for i in 0..self.entities.len() {
            if self.entities[i].is_valid {
                let rf = EntityRef {
                    genr: self.entities[i].generation,
                    idx: i,
                };
                out.push(rf);
            }
        }
        out
    }

    pub fn get_entity_with_name(&self, name: &str, from_pos: Option<Vec3>) -> Option<EntityRef> {
        if name.is_empty() {
            return None;
        }
        for (idx, i) in self.entities.iter().enumerate() {
            if i.is_valid && &*i.name == name {
                let rf = EntityRef {
                    idx,
                    genr: i.generation,
                };
                if let Some(p) = from_pos.clone() {
                    if self.get_observed_entity(rf, p).is_some() {
                        return Some(rf);
                    }
                } else {
                    return Some(rf);
                }
            }
        }
        None
    }

    pub fn calculate_entity_name(&self, base_name: &str) -> Arc<str> {
        if base_name.is_empty() {
            return base_name.to_string().into();
        }
        let mut found = false;
        for i in self.entities.iter() {
            if i.is_valid {
                if &*i.name == base_name {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return base_name.to_string().into();
        }
        let mut idx = 1;
        loop {
            found = false;
            let name = format!("{}_{}", base_name, idx);
            for i in self.entities.iter() {
                if i.is_valid {
                    if &*i.name == base_name {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                return name.into();
            }
            idx += 1;
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
            width: Fx::new(0),
            height: Fx::new(0),
            depth: Fx::new(0),
            components: ComponentData {
                components: HashMap::new(),
            },
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
            forward_vector: Vec3::new_i64(1, 0, 0),
            up_vector: Vec3::new_i64(0, 0, 1),
        }
    }
}

pub fn allegiance_commands(
    game: &GameState,
    allegiance: EntityAllegiance,
    get_entity_commands: impl Fn(&GameState, EntityRef, EntityAllegiance) -> Option<GameCmd>,
) -> Vec<GameCmd> {
    let mut cmds = Vec::new();
    let entities = game.get_all_entities();
    for i in entities {
        if let Some(et) = game.get_entity(i) {
            if (et.allegiance == allegiance) && (et.kind == EntityKind::Ship) {
                if let Some(cmd) = get_entity_commands(game, i, allegiance) {
                    cmds.push(cmd);
                }
            }
        }
    }
    cmds
}
