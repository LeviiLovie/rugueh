use minifb::{Window, WindowOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub const ASCII_MODE: &str = "ascii";
pub const GRAPHIC_MODE: &str = "graphic";

pub const COLOR_BLACK: u32 = 0x000000;
pub const COLOR_WHITE: u32 = 0xffffff;
pub const COLOR_RED: u32 = 0xff0000;
pub const COLOR_GREEN: u32 = 0x00ff00;
pub const COLOR_BLUE: u32 = 0x0000ff;
pub const COLOR_YELLOW: u32 = 0xffff00;
pub const COLOR_CYAN: u32 = 0x00ffff;
pub const COLOR_MAGENTA: u32 = 0xff00ff;

#[derive(Debug)]
pub struct WindowData {
    pub mode: String,
    pub width: usize,
    pub height: usize,
    pub title: String,
    pub fps: usize,
    pub buffer: Vec<u32>,
    pub window: Arc<Mutex<Window>>,
}

impl WindowData {
    pub fn new(mode: &str, width: usize, height: usize, title: &str, fps: usize) -> WindowData {
        WindowData {
            mode: String::from(mode),
            width,
            height,
            title: String::from(title),
            fps,
            buffer: vec![0; width * height],
            window: Arc::new(Mutex::new(Window::new(title, width, height, WindowOptions::default())
                .unwrap_or_else(|e| panic!("{}", e)))),
        }
    }

    pub fn init(&self) {
        let window_data = Arc::new(self.clone()); // Clone the WindowData and wrap it in an Arc

        // Spawn a separate thread to update the window continuously
        thread::spawn(move || {
            // Extract the cloned WindowData from the Arc
            let window_data = Arc::try_unwrap(window_data).unwrap();
            let mut window = window_data.window.lock().unwrap();

            while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
                window.update_with_buffer(&window_data.buffer, window_data.width, window_data.height).unwrap();
                thread::sleep(Duration::from_millis(16)); // Adjust the sleep duration as needed
            }
        });
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn set_pixels(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for i in x..(x + width) {
            for j in y..(y + height) {
                self.set_pixel(i, j, color);
            }
        }
    }
}
