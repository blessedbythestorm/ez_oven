mod oven_screen;

pub use oven_screen::Oven;

use eframe::epi;

pub trait Screen {
    fn create() -> Box<dyn Screen> where Self: Sized;
    fn initialize(&mut self);
    fn draw(&mut self, ctx: &egui::Context, frame: &epi::Frame) -> Option<Box<dyn Screen>>;
}