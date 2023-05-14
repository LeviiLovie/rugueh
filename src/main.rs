#![allow(dead_code)]
#![allow(unused_variables)]

mod window;

use window::WindowData;

fn main() {
    let mut game_window: WindowData = WindowData::new(window::ASCII_MODE, 500, 500, "Logs", 30);
    game_window.init();
    game_window.set_pixels(10, 10, 480, 480, 0xffffff);
}
