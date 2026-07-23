use serde::{Deserialize, Serialize};
use std::{
    collections::{HashSet, VecDeque},
    format,
    sync::Arc,
};

use crate::utils::{Fx, Orientation, Vec3};
pub const C: Fx = Fx::new(299_792_458);
pub const GAME_FRAME_DURATION: Fx = Fx::new_f64(0.01);

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct EntityRef {
    pub idx: u64,
    pub gn: u64,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum EntityKind {
    #[default]
    Object,
    Ship,
    Missile,
    Laser,
    Bullet,
    RailgunSlug,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    Laser,
    Missile,
    Cannon,
    Railgun,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EntityComponentKind {
    Cockpit,
    Engine,
    Laser,
    Cannon,
    Railgun,
    MissileTube,
    CrewCompartment,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub enum EntityMovementState {
    #[default]
    FreeFall,
    HoldPosition,
    Pursue {
        target: EntityRef,
    },
    Intercept {
        target: EntityRef,
    },
    Ram {
        target: EntityRef,
    },
    Accelerate {
        desired_velocity: Vec3,
    },
    MoveTo {
        point: Vec3,
    },
    LookAt {
        point: Vec3,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EntityCmd {
    FreeFall,
    HoldPosition,
    Pursue {
        target: EntityRef,
    },
    Intercept {
        target: EntityRef,
    },
    Ram {
        target: EntityRef,
    },
    Accelerate {
        desired_velocity: Vec3,
    },
    MoveTo {
        point: Vec3,
    },
    LookAtPoint {
        at: Vec3,
    },
    LookAtEntity {
        entity: EntityRef,
    },
    FireWeaponDirection {
        direction: Vec3,
        weapon: WeaponKind,
    },
    FireWeaponAtPoint {
        point: Vec3,
        weapon: WeaponKind,
    },
    FireWeapon {
        target: EntityRef,
        weapon: WeaponKind,
    },
    Repair {
        to_repair_facing: Facing,
        to_repair_component: EntityComponentKind,
    },
    Wait,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct EntityGameCmd {
    pub entity: EntityRef,
    pub cmd: EntityCmd,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entity {
    pub name: Arc<str>,
    pub kind: EntityKind,
    pub position: Vec3,
    pub velocity: Vec3,
    pub orientation: Orientation,
    pub mass: Fx,
    pub width: Fx,
    pub height: Fx,
    pub depth: Fx,
    pub health: i64,
    pub remaining_lifetime: i64,
    pub has_fuel: bool,
    pub max_fuel: i64,
    pub remaining_fuel: i64,
    pub ship_data: ShipData,
    pub movement_state: EntityMovementState,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
#[repr(usize)]
pub enum Facing {
    #[default]
    Forward = 0,
    Backward = 1,
    Left = 2,
    Right = 3,
    Up = 4,
    Down = 5,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct ShipComponentData {
    pub has_cockpit: bool,
    pub has_engine: bool,
    pub has_laser: bool,
    pub has_cannon: bool,
    pub has_railgun: bool,
    pub has_missile_tube: bool,
    pub has_crew_compartment: bool,
    pub armor: i64,
    pub max_cockpit_health: i64,
    pub max_engine_health: i64,
    pub max_laser_health: i64,
    pub max_cannon_health: i64,
    pub max_railgun_health: i64,
    pub max_missile_tube_health: i64,
    pub max_crew_compartment_health: i64,
    pub cockpit_health: i64,
    pub engine_health: i64,
    pub laser_health: i64,
    pub cannon_health: i64,
    pub railgun_health: i64,
    pub missile_tube_health: i64,
    pub crew_compartment_health: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ShipData {
    pub has_magazine: bool,
    pub max_missiles: i64,
    pub remaining_missiles: i64,
    pub max_bullets: i64,
    pub remaining_bullets: i64,
    pub max_railgun_slugs: i64,
    pub remaining_railgun_slugs: i64,
    pub components: [ShipComponentData; 6],
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EntityWrapper {
    pub generation: u64,
    pub entity: Option<Entity>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EntityCollection {
    pub entities: Box<[EntityWrapper]>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct GameState {
    pub event_buffer: VecDeque<GameEvent>,
    pub current_time_stamp: i64,
    pub state: EntityCollection,
    pub previous_states: VecDeque<EntityCollection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameEvent {
    pub location: Vec3,
    pub time_stamp: i64,
    pub data: GameEventData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameEventData {
    Damage {
        damager_name: Arc<str>,
        damaged_name: Arc<str>,
        damaged_kind: EntityKind,
        amount: i64,
    },
    Explosion {
        exploded_name: Arc<str>,
        exploded_kind: EntityKind,
        radius: Fx,
    },
    Destruction {
        destroyed_name: Arc<str>,
        exploded_kind: EntityKind,
    },
    WeaponDischarge {
        firer_name: Arc<str>,
        firer_kind: EntityKind,
        weapon_kind: WeaponKind,
        direction: Vec3,
    },
    AccelerationChange {
        entity_name: Arc<str>,
        entity_kind: EntityKind,
        change_in_direction: Vec3,
    },
}
impl ShipComponentData {
    pub const fn new() -> Self {
        Self {
            has_cockpit: false,
            has_engine: false,
            has_laser: false,
            has_cannon: false,
            has_railgun: false,
            has_missile_tube: false,
            has_crew_compartment: false,
            armor: 0,
            max_cockpit_health: 0,
            max_engine_health: 0,
            max_laser_health: 0,
            max_cannon_health: 0,
            max_railgun_health: 0,
            max_missile_tube_health: 0,
            max_crew_compartment_health: 0,
            cockpit_health: 0,
            engine_health: 0,
            laser_health: 0,
            cannon_health: 0,
            railgun_health: 0,
            missile_tube_health: 0,
            crew_compartment_health: 0,
        }
    }
}

impl ShipData {
    pub fn new() -> Self {
        Self {
            has_magazine: false,
            max_missiles: 0,
            remaining_missiles: 0,
            max_bullets: 0,
            remaining_bullets: 0,
            max_railgun_slugs: 0,
            remaining_railgun_slugs: 0,
            components: [ShipComponentData::new(); _],
        }
    }
}
impl Entity {
    pub fn new(position: Vec3) -> Self {
        Self {
            name: "".into(),
            kind: EntityKind::Object,
            position: position,
            velocity: Vec3::new_i64(0, 0, 0),
            orientation: Orientation::identity(),
            mass: 1.into(),
            width: 1.into(),
            height: 1.into(),
            depth: 1.into(),
            health: 1,
            remaining_lifetime: -1,
            has_fuel: false,
            max_fuel: 0,
            remaining_fuel: 0,
            ship_data: ShipData::new(),
            movement_state: EntityMovementState::FreeFall,
        }
    }
}

impl EntityCollection {
    pub fn get_entity(&self, et: EntityRef) -> Option<&Entity> {
        if et.idx as usize >= self.entities.len() {
            return None;
        }
        let rs = &self.entities[et.idx as usize];
        if rs.generation == et.gn {
            if let Some(t) = rs.entity.as_ref() {
                return Some(t);
            }
        }
        None
    }
}

impl GameState {
    pub fn get_entity_current(&self, et: EntityRef) -> Option<&Entity> {
        self.state.get_entity(et)
    }

    pub fn get_entity_rel(&self, et: EntityRef, pos: Vec3) -> Option<&Entity> {
        if let Some(t) = self.get_entity_current(et) {
            if t.position.distance(pos) / C < GAME_FRAME_DURATION {
                return Some(t);
            }
        }
        for (idx, i) in self.previous_states.iter().enumerate() {
            if let Some(t) = i.get_entity(et) {
                if t.position.distance(pos) / C < GAME_FRAME_DURATION * (idx as i64).into() {
                    return Some(t);
                }
            }
        }
        None
    }

    pub fn create_entity(&mut self, et: Entity) -> Option<EntityRef> {
        for i in 0..self.state.entities.len() {
            if self.state.entities[i].entity.is_none() {
                let gn = self.state.entities[i].generation.wrapping_add(1);
                self.state.entities[i].generation = gn;
                self.state.entities[i].entity = Some(et);
                return Some(EntityRef { idx: i as u64, gn });
            }
        }
        None
    }

    pub fn destroy_entity(&mut self, et: EntityRef) {
        if et.idx as usize >= self.state.entities.len() {
            return;
        }
        let t = &mut self.state.entities[et.idx as usize];
        if t.generation == et.gn {
            t.entity = None;
        }
    }

    pub fn get_all_current_entities(&self) -> Vec<EntityRef> {
        let mut out = Vec::new();
        for i in 0..self.state.entities.len() {
            if self.state.entities[i].entity.is_some() {
                let t = EntityRef {
                    idx: i as u64,
                    gn: self.state.entities[i].generation,
                };
                out.push(t);
            }
        }
        out
    }

    pub fn get_all_entities_visible_from(&self, pos: Vec3) -> Vec<EntityRef> {
        let mut out = Vec::new();
        let mut out_set = HashSet::new();
        for i in 0..self.state.entities.len() {
            if let Some(t) = self.state.entities[i].entity.as_ref() {
                if t.position.distance(pos) / C < GAME_FRAME_DURATION {
                    let rf = EntityRef {
                        idx: i as u64,
                        gn: self.state.entities[i].generation,
                    };
                    out_set.insert(rf);
                    out.push(rf);
                }
            }
        }
        for (idx, i) in self.previous_states.iter().enumerate() {
            for j in 0..i.entities.len() {
                let rf = EntityRef {
                    gn: i.entities[j].generation,
                    idx: j as u64,
                };
                if out_set.contains(&rf) {
                    continue;
                }
                if let Some(t) = i.entities[j].entity.as_ref() {
                    if t.position.distance(pos) / C < GAME_FRAME_DURATION * (idx as i64).into() {
                        out_set.insert(rf);
                        out.push(rf);
                    }
                }
            }
        }
        out
    }
}

impl Facing {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Facing::Forward => "forward",
            Facing::Backward => "backward",
            Facing::Left => "left",
            Facing::Right => "right",
            Facing::Up => "up",
            Facing::Down => "down",
        }
    }
    pub const fn as_comp_str(&self) -> &'static str {
        match self {
            Facing::Forward => "fore",
            Facing::Backward => "aft",
            Facing::Left => "port",
            Facing::Right => "starboard",
            Facing::Up => "top",
            Facing::Down => "bottom",
        }
    }
    pub const fn list() -> &'static [Facing] {
        static LIST: [Facing; 6] = [
            Facing::Forward,
            Facing::Backward,
            Facing::Left,
            Facing::Right,
            Facing::Up,
            Facing::Down,
        ];
        &LIST
    }
}

impl GameState {
    pub fn tick(&mut self, cmds: Vec<EntityGameCmd>) {
        for i in cmds {
            let ent = i.entity;
            let cmd = i.cmd;
            match cmd {
                EntityCmd::FreeFall => {}
                EntityCmd::HoldPosition => {}
                EntityCmd::Pursue { target } => {}
                EntityCmd::Intercept { target } => {}
                EntityCmd::Ram { target } => {}
                EntityCmd::Accelerate { desired_velocity } => {}
                EntityCmd::MoveTo { point } => {}
                EntityCmd::LookAtPoint { at } => {}
                EntityCmd::LookAtEntity { entity } => {}
                EntityCmd::FireWeaponDirection { direction, weapon } => {}
                EntityCmd::FireWeaponAtPoint { point, weapon } => {}
                EntityCmd::FireWeapon { target, weapon } => {}
                EntityCmd::Repair {
                    to_repair_facing,
                    to_repair_component,
                } => {}
                EntityCmd::Wait => {}
            }
        }
    }
}
