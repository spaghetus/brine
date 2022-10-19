use eframe::{
	egui::{self, Button, Layout, Ui},
	emath::Align,
	epaint::Vec2,
	App,
};

#[derive(Default)]
pub struct BrineGui;

impl App for BrineGui {
	fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
		egui::TopBottomPanel::top("status").show(ctx, |ui| {
			self.status_panel(ui);
		});
		egui::TopBottomPanel::bottom("toolbox").show(ctx, |ui| {
			self.toolbox(ui);
		});
	}
}

impl BrineGui {
	fn status_panel(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label("Brine");
			ui.separator();
			egui::ComboBox::from_id_source("view")
				.selected_text("Placeholder View Selector")
				.show_ui(ui, |ui| ui.label("Unimplemented"));
		});
	}

	fn toolbox(&mut self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.allocate_ui_with_layout(
				Vec2::new(200.0, 60.0),
				Layout::top_down(Align::Center),
				|ui| {
					egui::ComboBox::from_id_source("profile")
						.selected_text("Unimplemented Profile Selector")
						.show_ui(ui, |ui| {
							ui.label("Unimplemented");
						});
					ui.separator();
					ui.horizontal(|ui| {
						if ui.button("New").clicked() {
							// TODO
						}
						if ui.button("Edit").clicked() {
							// TODO
						}
					});
				},
			);
			ui.allocate_ui_with_layout(
				Vec2::new(ui.available_width() - 250.0, 60.0),
				Layout::top_down_justified(Align::Center),
				|ui| {
					if ui
						.add_sized(ui.available_size(), Button::new("Play!"))
						.clicked()
					{
						// TODO
					}
				},
			);
			ui.vertical(|ui| {
				ui.heading("Welcome, Username!");
				ui.label("Not ready to play");
				if ui.button("Switch User").clicked() {
					// TODO
				}
			})
		});
	}
}
