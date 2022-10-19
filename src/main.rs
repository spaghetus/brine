use brine::gui::BrineGui;
use eframe::{run_native, NativeOptions};

fn main() {
	run_native(
		"Brine",
		NativeOptions::default(),
		Box::new(|_cc| Box::new(BrineGui::default())),
	);
}
