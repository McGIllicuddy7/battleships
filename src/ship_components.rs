use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ComponentKind {
    FuelTank,
    GammaRayLaser,
    XRayLaser,
    BurningLaser,
    ChaffGun,
    GatlingGun,
    FusionReactor,
    Engine,
    Radiator,
    RailGun,
    TorpedoTube,
    CockPit,
    NuclearTorpedoMag,
    ConventionalTorpedoMag,
    DockingPort,
    CrewQuarters,
    ProximityExplosive,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Facing {
    Front,
    Back,
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Component {
    pub kind: ComponentKind,
    pub health: i64,
    pub max_health: i64,
    pub damage_threshold: i64,
    pub facing: Facing,
    pub item_count: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComponentData {
    pub components: HashMap<Arc<str>, Component>,
}

impl ComponentData {}
