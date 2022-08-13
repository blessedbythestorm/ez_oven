use crate::{screens, screens::Screen};

pub struct App {
    screen: Box<dyn Screen>,
}

impl App {
    pub fn new() -> Box<App> {
        Box::new(App {
            screen: screens::Oven::create(),
        })
    }
}

impl eframe::epi::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        if let Some(new_screen) = self.screen.draw(ctx, frame) {
            self.screen = new_screen;
        }
    }

    fn name(&self) -> &str {
        "Ez Oven"
    }
}