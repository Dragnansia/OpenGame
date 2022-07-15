use super::View;

pub struct Home;

impl View for Home {
    fn name(&self) -> &'static str {
        "Home"
    }

    fn init(&mut self) {}

    fn update(&self, ui: &mut eframe::egui::Ui) {
        ui.label("Home View");
    }
}
