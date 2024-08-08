use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn timeline_popup(&mut self, ctx: &egui::Context) {
        if self.show_timeline_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Timeline")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .min_size([300.0, 300.0])
                .max_size([300.0, 600.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            //
                        });
                        if ui.button("✖ Close").clicked() {
                            self.show_timeline_popup = false;
                        }
                    });
                });
            self.show_timeline_popup &= show_popup;
        }
    }
}
