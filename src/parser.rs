use std::sync::Arc;

use crate::{
    battleship::{EntityAllegiance, EntityKind, EntityRef, GameCmd, GameState},
    gui::gets,
    utils::Vec3,
};

pub fn local_player_get_entity_commands(
    game: &GameState,
    entity: EntityRef,
    player_allegiance: EntityAllegiance,
) -> Option<GameCmd> {
    loop {
        let input = gets();
        if let Some(cmd) = parse_cmd(game, entity, player_allegiance, &input) {
            match cmd {
                PlayerCmd::SetCourseFor { entity, to } => {
                    let out = GameCmd::SetCourseFor { entity, to };
                    todo!();
                    return Some(out);
                }
                PlayerCmd::SetCourseForEntity { entity, target } => {
                    let out = GameCmd::SetCourseForEntity { entity, target };
                    return Some(out);
                }
                PlayerCmd::Accelerate { entity, direction } => {
                    let out = GameCmd::Accelerate { entity, direction };
                    return Some(out);
                }
                PlayerCmd::FireWeapon {
                    entity,
                    at,
                    weapon_kind,
                } => {
                    let out = GameCmd::FireWeapon {
                        entity,
                        at,
                        weapon_kind: todo!(),
                    };
                    return Some(out);
                }
                PlayerCmd::FireWeaponAtPoint {
                    entity,
                    at,
                    weapon_kind,
                } => {
                    let out = GameCmd::FireWeaponAtPoint { entity, at };
                    return Some(out);
                }
                PlayerCmd::Repair { entity, component } => {
                    let out = GameCmd::Repair { entity, component };
                    return Some(out);
                }
                PlayerCmd::ShowStateFrom { entity, detailed } => {
                    show_state_from_entity(game, entity, detailed);
                    continue;
                }
                PlayerCmd::Pursue { entity, to_pursue } => {
                    let out = GameCmd::PursueEntity {
                        entity,
                        target: to_pursue,
                    };
                    return Some(out);
                }
                PlayerCmd::DoNothing => {
                    return None;
                }
                PlayerCmd::Help => {
                    todo!();
                }
            }
        }
    }
}

pub enum PlayerCmd {
    SetCourseFor {
        entity: EntityRef,
        to: Vec3,
    },
    SetCourseForEntity {
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
        weapon_kind: Arc<str>,
    },
    FireWeaponAtPoint {
        entity: EntityRef,
        at: Vec3,
        weapon_kind: Arc<str>,
    },
    Repair {
        entity: EntityRef,
        component: Arc<str>,
    },
    ShowStateFrom {
        entity: EntityRef,
        detailed: bool,
    },
    Pursue {
        entity: EntityRef,
        to_pursue: EntityRef,
    },
    DoNothing,
    Help,
}

pub fn parse_cmd(
    game: &GameState,
    entity: EntityRef,
    player_allegiance: EntityAllegiance,
    cmd: &str,
) -> Option<PlayerCmd> {
    _ = player_allegiance;
    let mut list = cmd.split_ascii_whitespace();
    let base_cmd = list.next()?;
    let player_pos = game.get_entity(entity)?.current_state.position;
    match base_cmd {
        "fire" => {
            let weapon = list.next()?;
            let ats = list.next()?;
            if ats != "at" {
                return None;
            }
            let c1 = list.next()?;
            if let Some(c2) = list.next() {
                let c3 = list.next()?;
                if list.next().is_some() {
                    return None;
                }
                let pos = Vec3::from_strs(c1, c2, c3)?;
                return Some(PlayerCmd::FireWeaponAtPoint {
                    entity,
                    at: pos,
                    weapon_kind: weapon.to_string().into(),
                });
            } else {
                let et = game.get_entity_with_name(c1, Some(player_pos))?;
                let out = PlayerCmd::FireWeapon {
                    entity,
                    at: et,
                    weapon_kind: weapon.to_string().into(),
                };
                return Some(out);
            }
        }
        "move" => {
            let to_contents = list.next()?;
            if to_contents != "to" {
                return None;
            }
            let c1 = list.next()?;
            if let Some(c2) = list.next() {
                let c3 = list.next()?;
                if list.next().is_some() {
                    return None;
                }
                let pos = Vec3::from_strs(c1, c2, c3)?;
                let out = PlayerCmd::SetCourseFor { entity, to: pos };
                return Some(out);
            } else {
                let et = game.get_entity_with_name(c1, Some(player_pos))?;
                let out = PlayerCmd::SetCourseForEntity { entity, target: et };
                return Some(out);
            }
        }
        "accelerate" => {
            let c1 = list.next()?;
            let c2 = list.next()?;
            let c3 = list.next()?;
            if list.next().is_some() {
                return None;
            }
            let acc = Vec3::from_strs(c1, c2, c3)?;
            return Some(PlayerCmd::Accelerate {
                entity,
                direction: acc,
            });
        }
        "set" => {
            let course = list.next()?;
            if course != "course" {
                return None;
            }
            let fors = list.next()?;
            if fors != "for" {
                return None;
            }
            let c1 = list.next()?;
            if let Some(c2) = list.next() {
                let c3 = list.next()?;
                if list.next().is_some() {
                    return None;
                }
                let pos = Vec3::from_strs(c1, c2, c3)?;
                return Some(PlayerCmd::SetCourseFor { entity, to: pos });
            } else {
                let et = game.get_entity_with_name(c1, Some(player_pos))?;
                return Some(PlayerCmd::SetCourseForEntity { entity, target: et });
            }
        }
        "repair" => {
            let comp = list.next()?;
            if list.next().is_some() {
                return None;
            }
            return Some(PlayerCmd::Repair {
                entity,
                component: comp.to_string().into(),
            });
        }
        "show" => {
            let detailed = if let Some(det) = list.next() {
                if det != "detailed" {
                    return None;
                }
                true
            } else {
                false
            };
            if list.next().is_some() {
                return None;
            }
            return Some(PlayerCmd::ShowStateFrom { entity, detailed });
        }
        "pursue" => {
            let target = list.next()?;
            let tg = game.get_entity_with_name(target, Some(player_pos))?;
            if list.next().is_some() {
                return None;
            }
            return Some(PlayerCmd::Pursue {
                entity,
                to_pursue: tg,
            });
        }
        "help" => {
            return Some(PlayerCmd::Help);
        }
        "wait" => {
            if list.next().is_some() {
                return None;
            }
            return Some(PlayerCmd::DoNothing);
        }
        "cancel" => {
            return None;
        }
        _ => {
            return None;
        }
    }
}

pub fn show_state_from_entity(game: &GameState, entity: EntityRef, detailed: bool) {
    let pos = game.get_entity(entity).unwrap().current_state.position;
    for i in game.get_all_entities() {
        let tmp = game.get_observed_entity(i, pos);
        if let Some(tmp) = tmp {
            if (tmp.kind == EntityKind::Ship || tmp.kind == EntityKind::Inanimate)
                || (detailed && tmp.kind == EntityKind::Missile)
            {}
            println!("{}:{:#?}", tmp.name, tmp.current_state);
        }
    }
}
