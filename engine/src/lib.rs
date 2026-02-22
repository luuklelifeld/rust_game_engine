mod gpu;
pub mod input;
mod runner;
mod texture_atlas;
mod util;

pub use input::{Input, Key};
pub use runner::run;
pub use texture_atlas::TextureAtlas;

pub struct EngineConfig {
    pub width: u32,
    pub height: u32,
}

pub trait Renderer {
    fn set_pixel(&mut self, x: u32, y: u32, red: u8, green: u8, blue: u8);
    fn clear(&mut self, red: u8, green: u8, blue: u8);
    fn fill_rect(&mut self, x: i32, y: i32, width: u32, height: u32, red: u8, green: u8, blue: u8);
    fn draw_sprite(&mut self, x: i32, y: i32, width: u32, height: u32, pixels: &[u8]);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub trait GameLoop {
    fn update(&mut self, input: &Input, delta_time: f32, screen_width: u32, screen_height: u32);
    fn draw(&self, renderer: &mut dyn Renderer);
}
