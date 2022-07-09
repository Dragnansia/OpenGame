//! Application struct for EGUI

use eframe::{egui, App};

#[derive(Default)]
pub struct OG;

impl App for OG {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_| {
            // Use panel for tab

            // Create mutiple view
        });
    }
}
