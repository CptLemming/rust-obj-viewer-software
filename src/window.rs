use glam::*;
use minifb::{MouseButton, MouseMode};

const X_MOVE_THRESHOLD: f32 = 10.0;
const Y_MOVE_THRESHOLD: f32 = 10.0;

pub struct Window {
    window: minifb::Window,
    buffer: FrameBuffer,
    zoom: f32,
    rotate_x: f32,
    rotate_y: f32,

    last_mouse_x: f32,
    last_mouse_y: f32,
}

impl Window {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let options = minifb::WindowOptions {
            resize: true,
            ..Default::default()
        };

        let window =
            minifb::Window::new(name, width, height, options).expect("Failed to create window");

        let buffer = FrameBuffer::new(width, height);

        Self {
            window,
            buffer,
            zoom: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
            last_mouse_x: 0.0,
            last_mouse_y: 0.0,
        }
    }

    pub fn buffer(&mut self) -> &mut FrameBuffer {
        &mut self.buffer
    }

    pub fn should_close(&self) -> bool {
        !self.window.is_open() || self.window.is_key_down(minifb::Key::Escape)
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn rotate(&self) -> Vec2 {
        Vec2::new(self.rotate_x, self.rotate_y)
    }

    pub fn display(&mut self) {
        if let Some((_scroll_x, scroll_y)) = self.window.get_scroll_wheel() {
            self.zoom += scroll_y / 10.0;
        }

        if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Discard) {
            if self.window.get_mouse_down(MouseButton::Left) {
                if (self.last_mouse_x - x).abs() > X_MOVE_THRESHOLD {
                    if x < self.last_mouse_x {
                        self.rotate_x += 0.05;
                    } else if x > self.last_mouse_x {
                        self.rotate_x -= 0.05;
                    }

                    self.last_mouse_x = x;
                }

                if (self.last_mouse_y - y).abs() > Y_MOVE_THRESHOLD {
                    if y < self.last_mouse_y {
                        self.rotate_y -= 1.0;
                    } else if y > self.last_mouse_y {
                        self.rotate_y += 1.0;
                    }

                    self.last_mouse_y = y;
                }
            }
        }

        self.window
            .update_with_buffer(&self.buffer.data, self.buffer.width, self.buffer.height)
            .expect("Failed to update window buffer");

        let (width, height) = self.window.get_size();

        if width != self.buffer.width || height != self.buffer.height {
            self.buffer = FrameBuffer::new(width, height);
        }
    }
}

pub struct FrameBuffer {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u32) {
        self.data[x + y * self.width] = value;
    }

    pub fn set_pixel_f32(&mut self, x: usize, y: usize, value: f32) {
        self.data[x + y * self.width] = (value * u32::MAX as f32) as u32;
    }

    pub fn get_pixel_f32(&mut self, x: usize, y: usize) -> f32 {
        self.data[x + y * self.width] as f32 / u32::MAX as f32
    }

    pub fn clear(&mut self, value: u32) {
        for i in 0..self.data.len() {
            self.data[i] = value;
        }
    }
}
