use raylib::drawing::RaylibDraw;
use serde::{Deserialize, Serialize};

pub mod battleship;
pub mod gui;
pub mod parser;
pub mod ship_components;
pub mod utils;
use gui::rl;
use rl::Color;

use crate::gui::{
    application_wrapper_done, application_wrapper_exit, draw_text, gets, left_panel_draw_text,
    right_panel_draw_text, terminal_clear,
};
pub fn main() {
    gui::run_gui_loop(|| {
        main_func();
        Ok(())
    })
    .unwrap();
}

pub fn main_func() {
    println!("hello world!:{}", 3);
    draw_text("henlo there", 10, 10, 16, Color::WHITE);
    left_panel_draw_text("this is a panel", 10, 10, 32, Color::WHITE);
    right_panel_draw_text("this is also a panel", 10, 10, 32, Color::WHITE);
    while !application_wrapper_done() {
        let input = gets();
        if input == "exit" {
            break;
        }
        if input == "clear" {
            terminal_clear();
            continue;
        }
        println!("echo:{}", input);
    }
    application_wrapper_exit();
}
