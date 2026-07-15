use crate::{create_static_object_set, utils::ObjectSet};
pub use RaylibTextureModeExt;
pub use raylib::prelude::*;
use std::sync::{Arc, Mutex};
pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;
pub struct Boundary {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub trait Widget {
    fn render(&self, draw: &mut RaylibTextureMode<RaylibHandle>, thread: &RaylibThread);
    fn update(&self, handle: &mut RaylibHandle, thread: &RaylibThread);
    fn get_min_bounds(&self) -> Boundary;
    fn get_max_bounds(&self) -> Boundary;
    fn get_bounds(&self) -> Boundary;
    fn set_bounds(&self, bounds: Boundary);
}
