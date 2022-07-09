use eframe::{egui::Context, Frame};

/// Basic trait to display tab content
pub trait View {
    /// Update/Display view
    fn update(&self, ctx: &Context, frame: &mut Frame);
}
