//! Application struct for EGUI

use crate::view::View;
use eframe::{egui, App};

#[derive(Default)]
pub struct OG {
    current_view: Option<Box<dyn View>>,
    views: Vec<Box<dyn View>>,
}

impl OG {
    pub fn new() -> Self {
        Self {
            current_view: None,
            views: vec![],
        }
    }
}

impl App for OG {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab panel")
            .resizable(false)
            .min_height(32f32)
            .show(ctx, |ui| {
                self.views.iter().for_each(|v| {
                    if ui.button(&v.name()).clicked() {
                        println!("New UI {}", v.name());
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Create mutiple view
            if let Some(view) = &self.current_view {
                view.update(ui);
            }
        });
    }
}
