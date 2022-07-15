use super::View;

/// View to install gaming dependencies
pub struct Gaming;

impl View for Gaming {
    fn name(&self) -> &'static str {
        "Gaming"
    }

    fn init(&mut self) {}

    fn update(&self, ui: &mut eframe::egui::Ui) {
        ui.label("Gaming View");
    }
}
