use eframe::{egui, run_native, App, NativeOptions};

#[derive(Default)]
struct OG;

impl App for OG {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_| {});
    }
}

fn main() {
    let options = NativeOptions::default();
    run_native("OpenGame", options, Box::new(|_| Box::new(OG::default())));
}
