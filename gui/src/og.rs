//! Application struct for EGUI

use crate::view::{gaming::Gaming, home::Home, View};
use eframe::{egui, App};

#[derive(Default)]
pub struct OG<'a, 'b: 'a> {
    current_view: Option<&'a dyn View>,
    views: Vec<&'b dyn View>,
}

impl<'a, 'b> OG<'a, 'b> {
    pub fn new() -> Self {
        let home_view = &Home;
        let views: Vec<&'b dyn View> = vec![home_view, &Gaming];

        Self {
            current_view: Some(home_view),
            views,
        }
    }
}

impl<'a, 'b> App for OG<'a, 'b> {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("tab panel")
            .resizable(false)
            .min_height(32f32)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    self.views.iter().for_each(|&v| {
                        if ui
                            .button(v.name())
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            self.current_view = Some(v);
                        }
                    });
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
