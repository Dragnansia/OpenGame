pub mod gaming;
pub mod home;

use eframe::egui::Ui;

/// Basic trait to display tab content
pub trait View {
    /// Return view name to display
    fn name(&self) -> &'static str;

    /// Init current view
    fn init(&mut self);

    /// Update/Display view
    fn update(&self, ui: &mut Ui);
}
