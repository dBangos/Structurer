use crate::save_load::source::update_source;
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("Add source to point")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    //If it's like a link make it a hyperlink
                    if self.points[&self.point_requesting_action_id].source != "No source set yet."
                        && (self.points[&self.point_requesting_action_id]
                            .source
                            .contains("www")
                            || self.points[&self.point_requesting_action_id]
                                .source
                                .contains("https"))
                    {
                        ui.hyperlink(self.points[&self.point_requesting_action_id].source.clone());
                    } else {
                        ui.label(self.points[&self.point_requesting_action_id].source.clone());
                    }
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(
                            &mut self
                                .points
                                .get_mut(&self.point_requesting_action_id)
                                .unwrap()
                                .source,
                        );
                        if ui.button("✅ Add Source").clicked() {
                            update_source(
                                self.project_directory.clone(),
                                self.points[&self.point_requesting_action_id].id.clone(),
                                self.points[&self.point_requesting_action_id].source.clone(),
                            );
                            self.show_source_popup = false;
                        }
                        if ui.button("✖ Cancel").clicked() {
                            self.show_source_popup = false;
                        }
                    });
                });
            });
    }
}
