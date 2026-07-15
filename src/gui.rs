use crate::{create_static_object_set, utils::ObjectSet};
pub use raylib::prelude as rl;
use raylib::{
    color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    prelude::Color,
    texture::Image,
};

pub const FONT_NAME: &'static str = "Ac437_IBM_VGA_9x16.ttf";
pub use rl::RaylibTextureModeExt;
use std::{
    collections::VecDeque,
    process::abort,
    sync::{Arc, Mutex},
    time::Duration,
};
pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;
pub struct Boundary {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct Gui {
    pub should_exit: bool,
    pub image_data: ImageData,
    pub left_panel_data: ImageData,
    pub right_panel_data: ImageData,
    pub terminal_data: TerminalData,
    pub font: rl::Font,
}
pub struct ImageData {
    pub bounds: Boundary,
    pub is_dirty: bool,
    pub is_buffer_swapping_enabled: bool,
    pub image: rl::Image,
    pub rendered_image: rl::Texture2D,
}
pub struct TerminalData {
    pub font_size: i32,
    pub bounds: Boundary,
    pub terminal_output: VecDeque<Arc<str>>,
    pub current_terminal_input: String,
    pub current_terminal_output: String,
    pub terminal_input_queue: VecDeque<Arc<str>>,
    pub cached_lines: Vec<Arc<str>>,
    pub is_cache_valid: bool,
    pub shift_up: usize,
    pub padding: i32,
}
unsafe impl Send for Gui {}
unsafe impl Sync for Gui {}

pub static GUI: Mutex<Option<Gui>> = Mutex::new(None);

pub fn with_gui<T>(func: impl FnOnce(&mut Gui) -> T) -> T {
    let mut gui = GUI.lock().unwrap();
    let gui_ref = gui.as_mut().unwrap();
    func(gui_ref)
}

pub fn run_gui_loop(
    main_function: impl FnOnce() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (mut handle, thread) = rl::RaylibBuilder::default()
        .size(1200, 800)
        .title("hello window")
        .build();
    {
        let mut tmp = handle.begin_drawing(&thread);
        tmp.clear_background(Color::BLACK);
        drop(tmp);
    }
    handle.set_target_fps(61);
    setup_gui(&mut handle, &thread);
    let psuedo_main_thread = std::thread::spawn(main_function);
    while !handle.window_should_close() {
        let mut done = false;
        let mut gui_guard = GUI.lock().unwrap();
        let gui = gui_guard.as_mut().unwrap();
        if gui.should_exit {
            done = true;
        }
        update_tty(&mut gui.terminal_data, &mut handle, &thread);
        update_panel(&mut gui.left_panel_data, &mut handle, &thread);
        update_panel(&mut gui.image_data, &mut handle, &thread);
        update_panel(&mut gui.right_panel_data, &mut handle, &thread);
        let mut draw = handle.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        render_panel(&gui.image_data, &mut draw);
        render_panel(&gui.left_panel_data, &mut draw);
        render_panel(&gui.right_panel_data, &mut draw);
        render_tty(&gui.terminal_data, &mut draw, &gui.font);
        drop(gui_guard);
        if done {
            break;
        }
    }
    with_gui(|gui| {
        gui.should_exit = true;
    });
    psuedo_main_thread.join().unwrap()
}

pub fn setup_gui(handle: &mut rl::RaylibHandle, thread: &rl::RaylibThread) {
    let img = rl::Image::gen_image_color(SCREEN_WIDTH, SCREEN_HEIGHT, Color::BLACK);
    let texture = handle.load_texture_from_image(thread, &img).unwrap();
    let img_data = ImageData {
        bounds: Boundary {
            x: 280,
            y: 20,
            w: 640,
            h: 480,
        },
        image: img,
        rendered_image: texture,
        is_buffer_swapping_enabled: false,
        is_dirty: false,
    };
    let (left_panel, right_panel) = {
        let panel_width = 260;
        let panel_height = 480;
        let left_image = rl::Image::gen_image_color(panel_width, panel_height, Color::BLACK);
        let left_texture = handle
            .load_texture_from_image(&thread, &left_image)
            .unwrap();
        let right_image = rl::Image::gen_image_color(panel_width, panel_height, Color::BLACK);
        let right_texture = handle
            .load_texture_from_image(&thread, &left_image)
            .unwrap();
        let lp = ImageData {
            bounds: Boundary {
                x: 10,
                y: 20,
                w: panel_width,
                h: panel_height,
            },
            is_buffer_swapping_enabled: false,
            is_dirty: false,
            image: left_image,
            rendered_image: left_texture,
        };
        let rp = ImageData {
            bounds: Boundary {
                x: 930,
                y: 20,
                w: panel_width,
                h: panel_height,
            },
            is_buffer_swapping_enabled: false,
            is_dirty: false,
            image: right_image,
            rendered_image: right_texture,
        };
        (lp, rp)
    };
    let tty_data = TerminalData {
        padding: 4,
        shift_up: 0,
        font_size: 24,
        bounds: Boundary {
            x: 10,
            y: 520,
            w: 1180,
            h: 260,
        },
        terminal_input_queue: VecDeque::new(),
        terminal_output: VecDeque::new(),
        current_terminal_input: String::new(),
        current_terminal_output: String::new(),
        cached_lines: Vec::new(),
        is_cache_valid: false,
    };
    let font = handle.load_font(&thread, FONT_NAME).unwrap();

    let gui = Gui {
        should_exit: false,
        image_data: img_data,
        terminal_data: tty_data,
        left_panel_data: left_panel,
        right_panel_data: right_panel,
        font,
    };
    *GUI.lock().unwrap() = Some(gui);
}

pub fn render_panel(panel: &ImageData, draw: &mut RaylibDrawHandle) {
    draw.draw_rectangle_lines(
        panel.bounds.x - 1,
        panel.bounds.y - 1,
        panel.bounds.w + 2,
        panel.bounds.h + 2,
        Color::GREEN,
    );
    draw.draw_texture(
        &panel.rendered_image,
        panel.bounds.x,
        panel.bounds.y,
        Color::WHITE,
    );
}

pub fn render_tty(tty: &TerminalData, draw: &mut RaylibDrawHandle, font: &rl::Font) {
    let padding = 4;
    draw.draw_rectangle_lines(
        tty.bounds.x - 1,
        tty.bounds.y - 1,
        tty.bounds.w + 2,
        tty.bounds.h + 2,
        Color::GREEN,
    );
    let mut dy = 0;
    let font_size = tty.font_size;
    let lc = (tty.bounds.h / (tty.font_size + tty.padding)) as usize;
    let start = if lc + tty.shift_up < tty.cached_lines.len() {
        tty.cached_lines.len() - lc - tty.shift_up
    } else {
        0
    };
    let end = if (start + lc) > tty.cached_lines.len() {
        tty.cached_lines.len()
    } else {
        start + lc
    };
    let lines = &tty.cached_lines[start..end];
    for i in lines {
        draw.draw_text_pro(
            font,
            i,
            rl::Vector2::new(
                (tty.bounds.x + padding) as f32,
                (tty.bounds.y + dy + padding) as f32,
            ),
            rl::Vector2::zero(),
            0.,
            font_size as f32,
            1.,
            Color::GREEN,
        );
        dy += font_size + padding;
    }
}

pub fn tty_get_lines(tty: &TerminalData) -> Vec<Arc<str>> {
    let font_size = tty.font_size;
    let mut out = Vec::new();
    for i in &tty.terminal_output {
        out.push(i.clone());
    }
    if !tty.current_terminal_output.is_empty() {
        let tmp =
            split_text_to_lines_to_render(&tty.current_terminal_output, font_size, tty.bounds.w);
        for i in tmp {
            out.push(i.clone().into());
        }
    }
    let mut first = "$:";
    if tty.current_terminal_input.is_empty() {
        out.push(first.to_string().into());
    }
    for i in split_text_to_lines_to_render(&tty.current_terminal_input, font_size, tty.bounds.w) {
        out.push(format!("{}{}", first, &i).into());
        first = "";
    }
    out
}

pub fn update_tty(
    tty: &mut TerminalData,
    handle: &mut rl::RaylibHandle,
    thread: &rl::RaylibThread,
) {
    use raylib::prelude::KeyboardKey;
    _ = thread;
    if let Some(c) = handle.get_char_pressed() {
        tty.is_cache_valid = false;
        tty.current_terminal_input.push(c);
    }
    if handle.is_key_pressed(KeyboardKey::KEY_ENTER) {
        tty.is_cache_valid = false;
        tty.terminal_input_queue
            .push_back(tty.current_terminal_input.clone().into());
        for i in
            split_text_to_lines_to_render(&tty.current_terminal_input, tty.font_size, tty.bounds.w)
        {
            tty.terminal_output.push_back(i.into());
        }
        tty.current_terminal_input.clear();
    }
    if handle.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
        tty.is_cache_valid = false;
        tty.current_terminal_input.pop();
    }
    if handle.is_key_pressed_repeat(KeyboardKey::KEY_UP)
        || handle.is_key_pressed(KeyboardKey::KEY_UP)
    {
        tty.shift_up += 1;
    }
    if handle.is_key_pressed_repeat(KeyboardKey::KEY_DOWN)
        || handle.is_key_pressed(KeyboardKey::KEY_DOWN)
    {
        if tty.shift_up > 0 {
            tty.shift_up -= 1;
        }
    }
    if !tty.is_cache_valid {
        tty.cached_lines = tty_get_lines(tty);
        tty.is_cache_valid = true;
        let lc = (tty.bounds.h / (tty.font_size + tty.padding)) as usize;
        let start = if lc < tty.cached_lines.len() {
            tty.cached_lines.len() - lc
        } else {
            0
        };
        if tty.shift_up > start {
            tty.shift_up = start;
            if tty.shift_up > 0 {
                tty.shift_up -= 1;
            }
        }
    }
    if tty.terminal_input_queue.len() > 20 {
        for _ in 0..tty.terminal_input_queue.len() - 20 {
            tty.terminal_input_queue.pop_front();
        }
    }
    if tty.terminal_output.len() > 100 {
        for _ in 0..tty.terminal_output.len() - 100 {
            tty.terminal_output.pop_front();
        }
    }
}

pub fn update_panel(
    panel: &mut ImageData,
    handle: &mut rl::RaylibHandle,
    thread: &rl::RaylibThread,
) {
    if panel.is_dirty {
        let tex = handle
            .load_texture_from_image(&thread, &panel.image)
            .unwrap();
        panel.rendered_image = tex;
        panel.is_dirty = false;
    }
}

pub fn split_text_to_lines_to_render(l: &str, font_size: i32, max_width: i32) -> Vec<String> {
    let char_width = ((font_size as f32 * 14.9) / 32.).floor() as i32;
    let mut current = String::new();
    let mut out = Vec::new();
    let mut dx = 0;
    for i in l.chars() {
        if i == '\r' {
            continue;
        }
        if i == '\n' || (dx + char_width > max_width && i != '\n') {
            out.push(current.clone());
            current.clear();
            dx = 0;
            if i != '\n' {
                current.push(i);
            }
        } else {
            dx += char_width;
            current.push(i);
        }
    }
    if !current.is_empty() {
        out.push(current);
    }
    out
}

pub fn write_to_tty(tty: &mut TerminalData, to_write: &str) {
    for i in to_write.chars() {
        if i == '\n' {
            let list = split_text_to_lines_to_render(
                &tty.current_terminal_output,
                tty.font_size,
                tty.bounds.w,
            );
            for i in list {
                tty.terminal_output.push_back(i.into());
            }
            tty.current_terminal_output.clear();
        } else {
            tty.current_terminal_output.push(i);
        }
    }
    let sts =
        split_text_to_lines_to_render(&tty.current_terminal_output, tty.font_size, tty.bounds.w);
    if sts.len() > 1 {
        for i in 0..sts.len() - 1 {
            tty.terminal_output.push_back(sts[i].clone().into());
        }
        tty.current_terminal_output = sts[sts.len() - 1].clone();
    }
    tty.is_cache_valid = false;
}

pub fn write(to_write: &str) {
    with_gui(|gui| {
        write_to_tty(&mut gui.terminal_data, to_write);
    })
}

#[macro_export]
macro_rules! print {
    ($fmt:literal $(,)?$($args:expr $(,)?)*) => {
        crate::gui::write(&format!($fmt, $($args,)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:literal $(,)?$($args:expr $(,)?)*) => {
        crate::gui::write(&(format!($fmt, $($args,)*)+"\n"))
    };
}

pub fn gets() -> String {
    loop {
        let mut done = false;
        let g = with_gui(|gui| {
            if gui.should_exit {
                done = true;
            }
            gui.terminal_data.terminal_input_queue.pop_front()
        });
        if let Some(t) = g {
            return t.to_string();
        }
        if done {
            return String::new();
        }
        std::thread::yield_now();
    }
}

pub fn draw_call(to_run: impl FnOnce(&mut Image)) {
    with_gui(|gui| {
        to_run(&mut gui.image_data.image);
        if !gui.image_data.is_buffer_swapping_enabled {
            gui.image_data.is_dirty = true;
        }
    });
}

pub fn left_panel_draw_call(to_run: impl FnOnce(&mut Image)) {
    with_gui(|gui| {
        to_run(&mut gui.left_panel_data.image);
        if !gui.left_panel_data.is_buffer_swapping_enabled {
            gui.left_panel_data.is_dirty = true;
        }
    });
}
pub fn right_panel_draw_call(to_run: impl FnOnce(&mut Image)) {
    with_gui(|gui| {
        to_run(&mut gui.right_panel_data.image);
        if !gui.right_panel_data.is_buffer_swapping_enabled {
            gui.right_panel_data.is_dirty = true;
        }
    });
}

pub fn draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
    draw_call(|img| {
        img.draw_line(start_x, start_y, end_x, end_y, color);
    });
}

pub fn draw_rectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw_call(|img| {
        img.draw_rectangle(x, y, w, h, color);
    });
}

pub fn draw_text_no_wrap(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        gui.image_data.image.draw_text_ex(
            &gui.font,
            text,
            rl::Vector2::new(x as f32, y as f32),
            text_height as f32,
            1. as f32,
            color,
        );
        if !gui.image_data.is_buffer_swapping_enabled {
            gui.image_data.is_dirty = true;
        }
    });
}

pub fn draw_circle(x: i32, y: i32, rad: i32, color: Color) {
    draw_call(|img| {
        img.draw_circle(x, y, rad, color);
    });
}

pub fn draw_triangle(p0: rl::Vector2, p1: rl::Vector2, p2: rl::Vector2, color: Color) {
    draw_call(|img| {
        img.draw_triangle(p0, p1, p2, color);
    });
}

pub fn draw_pixel(x: i32, y: i32, color: Color) {
    draw_call(|img| {
        img.draw_pixel(x, y, color);
    });
}

pub fn clear_background(color: Color) {
    draw_call(|img| {
        img.clear_background(color);
    });
}

pub fn left_panel_draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
    left_panel_draw_call(|img| {
        img.draw_line(start_x, start_y, end_x, end_y, color);
    });
}

pub fn left_panel_draw_rectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    left_panel_draw_call(|img| {
        img.draw_rectangle(x, y, w, h, color);
    });
}

pub fn left_panel_draw_text_no_wrap(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        gui.left_panel_data.image.draw_text_ex(
            &gui.font,
            text,
            rl::Vector2::new(x as f32, y as f32),
            text_height as f32,
            1. as f32,
            color,
        );
        if !gui.left_panel_data.is_buffer_swapping_enabled {
            gui.left_panel_data.is_dirty = true;
        }
    });
}

pub fn left_panel_draw_circle(x: i32, y: i32, rad: i32, color: Color) {
    left_panel_draw_call(|img| {
        img.draw_circle(x, y, rad, color);
    });
}

pub fn left_panel_draw_triangle(p0: rl::Vector2, p1: rl::Vector2, p2: rl::Vector2, color: Color) {
    left_panel_draw_call(|img| {
        img.draw_triangle(p0, p1, p2, color);
    });
}

pub fn left_panel_draw_pixel(x: i32, y: i32, color: Color) {
    left_panel_draw_call(|img| {
        img.draw_pixel(x, y, color);
    });
}

pub fn left_panel_clear_background(color: Color) {
    left_panel_draw_call(|img| {
        img.clear_background(color);
    });
}

pub fn right_panel_draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
    right_panel_draw_call(|img| {
        img.draw_line(start_x, start_y, end_x, end_y, color);
    });
}

pub fn right_panel_draw_rectangle(x: i32, y: i32, w: i32, h: i32, color: Color) {
    right_panel_draw_call(|img| {
        img.draw_rectangle(x, y, w, h, color);
    });
}

pub fn right_panel_draw_text_no_wrap(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        gui.right_panel_data.image.draw_text_ex(
            &gui.font,
            text,
            rl::Vector2::new(x as f32, y as f32),
            text_height as f32,
            1. as f32,
            color,
        );
        if !gui.right_panel_data.is_buffer_swapping_enabled {
            gui.right_panel_data.is_dirty = true;
        }
    });
}

pub fn right_panel_draw_circle(x: i32, y: i32, rad: i32, color: Color) {
    right_panel_draw_call(|img| {
        img.draw_circle(x, y, rad, color);
    });
}

pub fn right_panel_draw_triangle(p0: rl::Vector2, p1: rl::Vector2, p2: rl::Vector2, color: Color) {
    right_panel_draw_call(|img| {
        img.draw_triangle(p0, p1, p2, color);
    });
}

pub fn right_panel_draw_pixel(x: i32, y: i32, color: Color) {
    right_panel_draw_call(|img| {
        img.draw_pixel(x, y, color);
    });
}

pub fn right_panel_clear_background(color: Color) {
    right_panel_draw_call(|img| {
        img.clear_background(color);
    });
}

pub fn enable_buffer_swapping() {
    with_gui(|gui| {
        gui.image_data.is_buffer_swapping_enabled = true;
    });
}
pub fn disable_buffer_swapping() {
    with_gui(|gui| {
        gui.image_data.is_buffer_swapping_enabled = false;
    });
}

pub fn swap_buffers() {
    with_gui(|gui| {
        gui.image_data.is_dirty = true;
    });
}

pub fn left_panel_enable_buffer_swapping() {
    with_gui(|gui| {
        gui.left_panel_data.is_buffer_swapping_enabled = true;
    });
}
pub fn left_panel_disable_buffer_swapping() {
    with_gui(|gui| {
        gui.left_panel_data.is_buffer_swapping_enabled = false;
    });
}

pub fn left_panel_swap_buffers() {
    with_gui(|gui| {
        gui.left_panel_data.is_dirty = true;
    });
}

pub fn right_panel_enable_buffer_swapping() {
    with_gui(|gui| {
        gui.right_panel_data.is_buffer_swapping_enabled = true;
    });
}
pub fn right_panel_disable_buffer_swapping() {
    with_gui(|gui| {
        gui.right_panel_data.is_buffer_swapping_enabled = false;
    });
}

pub fn right_panel_swap_buffers() {
    with_gui(|gui| {
        gui.right_panel_data.is_dirty = true;
    });
}

pub fn terminal_clear() {
    with_gui(|gui| {
        gui.terminal_data.cached_lines.clear();
        gui.terminal_data.current_terminal_input.clear();
        gui.terminal_data.current_terminal_output.clear();
        gui.terminal_data.is_cache_valid = false;
        gui.terminal_data.terminal_input_queue.clear();
        gui.terminal_data.terminal_output.clear();
        gui.terminal_data.shift_up = 0;
    })
}

pub fn application_wrapper_done() -> bool {
    with_gui(|gui| gui.should_exit)
}
pub fn application_wrapper_exit() {
    with_gui(|gui| {
        gui.should_exit = true;
    });
}

pub fn draw_text(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        let mut dy = 0;
        let max_w = gui.image_data.bounds.w - x;
        if max_w < 0 {
            return;
        }
        let list = split_text_to_lines_to_render(text, text_height, max_w);
        for i in list {
            gui.image_data.image.draw_text_ex(
                &gui.font,
                &i,
                rl::Vector2::new(x as f32, (y + dy) as f32),
                text_height as f32,
                1. as f32,
                color,
            );
            dy += text_height;
        }
        if !gui.image_data.is_buffer_swapping_enabled {
            gui.image_data.is_dirty = true;
        }
    });
}

pub fn left_panel_draw_text(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        let mut dy = 0;
        let max_w = gui.left_panel_data.bounds.w - x;
        if max_w < 0 {
            return;
        }
        let list = split_text_to_lines_to_render(text, text_height, max_w);
        for i in list {
            gui.left_panel_data.image.draw_text_ex(
                &gui.font,
                &i,
                rl::Vector2::new(x as f32, (y + dy) as f32),
                text_height as f32,
                1. as f32,
                color,
            );
            dy += text_height;
        }
        if !gui.left_panel_data.is_buffer_swapping_enabled {
            gui.left_panel_data.is_dirty = true;
        }
    });
}

pub fn right_panel_draw_text(text: &str, x: i32, y: i32, text_height: i32, color: Color) {
    with_gui(|gui| {
        let mut dy = 0;
        let max_w = gui.right_panel_data.bounds.w - x;
        if max_w < 0 {
            return;
        }
        let list = split_text_to_lines_to_render(text, text_height, max_w);
        for i in list {
            gui.right_panel_data.image.draw_text_ex(
                &gui.font,
                &i,
                rl::Vector2::new(x as f32, (y + dy) as f32),
                text_height as f32,
                1. as f32,
                color,
            );
            dy += text_height;
        }
        if !gui.right_panel_data.is_buffer_swapping_enabled {
            gui.right_panel_data.is_dirty = true;
        }
    });
}
