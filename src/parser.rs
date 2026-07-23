use crate::{
    ship::{EntityCmd, EntityComponentKind, EntityRef, Facing, GameState, WeaponKind},
    utils::Vec3,
};

pub fn parse_entity_cmd(
    game: &GameState,
    entity: EntityRef,
    cmd: &str,
) -> Result<EntityCmd, String> {
    let cmds: Vec<&str> = cmd.split_whitespace().collect();
    if cmds.len() < 1 {
        return Err(format!("empty command"));
    }
    let base = cmds[0];
    match base {
        "fall" | "freefall" => {
            if cmds.len() > 1 {
                return Err(format!("{:#?} is not a valid command", cmd));
            }
            return Ok(EntityCmd::FreeFall);
        }
        "hold" | "hold-position" => {
            if cmds.len() > 1 {
                return Err(format!("{:#?} is not a valid command", cmd));
            }
            return Ok(EntityCmd::HoldPosition);
        }
        "pursue" | "follow" => {
            if cmds.len() > 2 {
                return Err(format!("{:#?} is not a valid command", cmd));
            }
            let target = parse_entity(game, entity, cmds[2])?;
            return Ok(EntityCmd::Pursue { target });
        }
        "intercept" => {
            if cmds.len() > 2 {
                return Err(format!("{:#?} is not a valid command", cmd));
            }
            let target = parse_entity(game, entity, cmds[2])?;
            return Ok(EntityCmd::Intercept { target });
        }
        "ram" => {
            if cmds.len() > 2 {
                return Err(format!("{:#?} is not a valid command", cmd));
            }
            let target = parse_entity(game, entity, cmds[2])?;
            return Ok(EntityCmd::Ram { target });
        }
        "accelerate" => {
            if cmds.len() == 4 {
                let vel = parse_vec3(&cmds[1..])?;
                return Ok(EntityCmd::Accelerate {
                    desired_velocity: vel,
                });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "move-to" => {
            if cmds.len() == 4 {
                let point = parse_vec3(&cmds[1..])?;
                return Ok(EntityCmd::MoveTo { point });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "look-at" => {
            if cmds.len() == 4 {
                let point = parse_vec3(&cmds[1..])?;
                return Ok(EntityCmd::LookAtPoint { at: point });
            } else if cmds.len() == 2 {
                let et = parse_entity(game, entity, cmds[1])?;
                return Ok(EntityCmd::LookAtEntity { entity: et });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "fire" => {
            if cmds.len() == 5 {
                let weapon = parse_weapon_kind(&cmds[1])?;
                let point = parse_vec3(&cmds[2..])?;
                return Ok(EntityCmd::FireWeaponAtPoint { point, weapon });
            } else if cmds.len() == 3 {
                let weapon = parse_weapon_kind(&cmds[1])?;
                let target = parse_entity(game, entity, &cmds[2])?;
                return Ok(EntityCmd::FireWeapon { target, weapon });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "fire-in-direction" => {
            if cmds.len() == 5 {
                let weapon = parse_weapon_kind(&cmds[1])?;
                let direction = parse_vec3(&cmds[2..])?;
                return Ok(EntityCmd::FireWeaponDirection { direction, weapon });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "repair" => {
            if cmds.len() == 3 {
                let facing = parse_facing(cmds[1])?;
                let component = parse_entity_component_kind(&cmds[2])?;
                return Ok(EntityCmd::Repair {
                    to_repair_facing: facing,
                    to_repair_component: component,
                });
            } else {
                return Err(format!("{} is not a valid command", cmd));
            }
        }
        "wait" | "_" | "-" => {
            if cmds.len() > 1 {
                return Err(format!("{} is not a valid command", cmd));
            } else {
                return Ok(EntityCmd::Wait);
            }
        }
        "help" | "please" | "fuck" | "shit" | "god" | "gods" | "egads" | "pain" => {
            return Err(
                "to play game enter commands as a list of space seperated strings listed below, values in paretheses are to be entered as numbers, values in square brackets are to be entered as a names. values seperated with a '|' both do the same thing. commands with extraneous words after a complete command are considered erroneous.
                    fall|freefall: allow ship to continue along a ballistic trajectory in freefall(turns off engines essentially).
                    hold|hold-position: order a ship to halt and hold its current position(holds the position where it stops not where the command is entered) .
                    pursue|follow [entity]: set ship to follow an entity.
                    intercept [entity]: set ship to intercept an entity.
                    ram [entity]: set ship to ram an entity
                    accelerate (velocity x) (velocity y) (velocity z): sets ship to accelerate towards the  desired velocity.
                    move-to (position x) (position y) (position z): sets ship to move to a point.
                    look-at (position x) (position y) (position z): 
                    sets ship to look at a point . 
                    look-at [entity]: sets ship to look at an entity.
                    fire [weapon] (point x) (point y) (point z):ship tries to fire weapon at a point.
                    fire [weapon] [entity]: ship tries to fire weapon at an entity.
                    fire-in-direction [weapon] (x direction) (y direction) (z direction): ship tries to fire weapon in a direction
                    wait | _ | - : wait, continuing whatever action ship was taking before this was done.
                    help: display this message
                    list-weapons: list the weapons on the ship as well as ammunition for it if applicable(note ammunition pools for weapons of the same kind are shared).
                    list-components: list the components of the ship as well as their respective healths. 
            "
                .into(),
            );
        }
        "list-weapons" => {
            let et = game.get_entity_current(entity).unwrap();
            let mut out = String::new();
            for i in Facing::list() {
                let l = &et.ship_data.components[*i as usize];
                if l.has_cannon {
                    out += &format!(
                        "\t{} cannon(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_bullets,
                        et.ship_data.max_bullets,
                        l.cannon_health,
                        l.max_cannon_health
                    );
                }
                if l.has_laser {
                    out += &format!(
                        "\t{} laser(health:{}/{})\n",
                        i.as_comp_str(),
                        l.laser_health,
                        l.max_laser_health
                    );
                }
                if l.has_missile_tube {
                    out += &format!(
                        "\t{} missile-tube(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_missiles,
                        et.ship_data.max_missiles,
                        l.missile_tube_health,
                        l.max_missile_tube_health,
                    );
                }
                if l.has_railgun {
                    out += &format!(
                        "\t{} railgun(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_railgun_slugs,
                        et.ship_data.max_railgun_slugs,
                        l.railgun_health,
                        l.max_railgun_health,
                    );
                }
            }
            return Err(out);
        }
        "list-components" => {
            let et = game.get_entity_current(entity).unwrap();
            let mut out = String::new();
            for i in Facing::list() {
                let l = &et.ship_data.components[*i as usize];
                if l.has_cockpit {
                    out += &format!(
                        "\t{} cockpit(health:{}/{})\n",
                        i.as_comp_str(),
                        l.cockpit_health,
                        l.max_cockpit_health
                    );
                }
                if l.has_engine {
                    out += &format!(
                        "\t{} engine(health:{}/{})\n",
                        i.as_comp_str(),
                        l.engine_health,
                        l.max_engine_health,
                    );
                }
                if l.has_crew_compartment {
                    out += &format!(
                        "\t{} crew-compartment(health:{}/{})\n",
                        i.as_comp_str(),
                        l.crew_compartment_health,
                        l.max_crew_compartment_health
                    );
                }
                if l.has_cannon {
                    out += &format!(
                        "\t{} cannon(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_bullets,
                        et.ship_data.max_bullets,
                        l.cannon_health,
                        l.max_cannon_health
                    );
                }
                if l.has_laser {
                    out += &format!(
                        "\t{} laser(health:{}/{})\n",
                        i.as_comp_str(),
                        l.laser_health,
                        l.max_laser_health
                    );
                }
                if l.has_missile_tube {
                    out += &format!(
                        "\t{} missile-tube(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_missiles,
                        et.ship_data.max_missiles,
                        l.missile_tube_health,
                        l.max_missile_tube_health,
                    );
                }
                if l.has_railgun {
                    out += &format!(
                        "\t{} railgun(ammunition: {}/{}, health:{}/{})\n",
                        i.as_comp_str(),
                        et.ship_data.remaining_railgun_slugs,
                        et.ship_data.max_railgun_slugs,
                        l.railgun_health,
                        l.max_railgun_health,
                    );
                }
            }
            return Err(out);
        }
        _ => return Err(format!("{} is not a valid command", base)),
    }
}
pub fn parse_facing(st: &str) -> Result<Facing, String> {
    match st {
        "forward" | "fore" => Ok(Facing::Forward),
        "backward" | "aft" => Ok(Facing::Backward),
        "left" | "port" => Ok(Facing::Left),
        "right" | "starboard" => Ok(Facing::Right),
        "up" | "top" => Ok(Facing::Up),
        "down" | "bottom" => Ok(Facing::Down),
        _ => Err(format!("{} is not a valid facing", st)),
    }
}

pub fn parse_weapon_kind(st: &str) -> Result<WeaponKind, String> {
    match st {
        "laser" => Ok(WeaponKind::Laser),
        "cannon" => Ok(WeaponKind::Cannon),
        "missile" => Ok(WeaponKind::Missile),
        "railgun" => Ok(WeaponKind::Railgun),
        _ => Err(format!("{} is not a valid kind of weapon", st)),
    }
}

pub fn parse_entity_component_kind(st: &str) -> Result<EntityComponentKind, String> {
    match st {
        "cockpit" => Ok(EntityComponentKind::Cockpit),
        "engine" => Ok(EntityComponentKind::Engine),
        "laser" => Ok(EntityComponentKind::Laser),
        "cannon" => Ok(EntityComponentKind::Cannon),
        "railgun" => Ok(EntityComponentKind::Railgun),
        "missile-tube" => Ok(EntityComponentKind::MissileTube),
        "crew-compartment" => Ok(EntityComponentKind::CrewCompartment),
        _ => Err(format!("{} is not a valid kind of entity component", st)),
    }
}

pub fn parse_vec3(st: &[&str]) -> Result<Vec3, String> {
    if st.len() != 3 {
        return Err(format!("{:#?} is not a valid vec3", st));
    }
    if let Some(t) = Vec3::from_strs(st[0], st[1], st[2]) {
        return Ok(t);
    }
    return Err(format!("{:#?} is not a valid vec3", st));
}

pub fn parse_entity(
    game: &GameState,
    base_entity: EntityRef,
    st: &str,
) -> Result<EntityRef, String> {
    let Some(et) = game.get_entity_current(base_entity) else {
        return Err(format!(
            "attempted to get entities from an invalid entity:{:#?}",
            base_entity
        ));
    };
    for i in game.get_all_entities_visible_from(et.position) {
        let t = game.get_entity_current(i).unwrap();
        if t.name.as_ref() == st {
            return Ok(i);
        }
    }
    return Err(format!("{} is not a valid entity", st));
}
