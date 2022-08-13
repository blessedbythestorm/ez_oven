mod screens;
mod app;

use eframe::NativeOptions;


fn main() {
    let options = NativeOptions {
        decorated: true,
        resizable: false,
        transparent: true,
        initial_window_size: Some(
            egui::Vec2::new(
                400.0,
                475.0,
            )),
        ..Default::default()
    };

    eframe::run_native(
        app::App::new(),
        options,
    );
}
