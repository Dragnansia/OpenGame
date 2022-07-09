mod og;
mod view;

use eframe::{run_native, NativeOptions};
use og::OG;

fn main() {
    let options = NativeOptions::default();
    run_native("OpenGame", options, Box::new(|_| Box::new(OG::default())));
}
