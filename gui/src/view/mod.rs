use eframe::egui::Ui;

/// Basic trait to display tab content
pub trait View {
    /// Return view name to display
    fn name(&self) -> String;

    /// Update/Display view
    fn update(&self, ui: &Ui);
}
