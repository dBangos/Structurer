use crate::save_load::source::update_source;
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("Add source to point")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    //If it's like a link make it a hyperlink
                    if self.current_points[self.point_requesting_action_index].source
                        != "No source set yet."
                        && (self.current_points[self.point_requesting_action_index]
                            .source
                            .contains("www")
                            || self.current_points[self.point_requesting_action_index]
                                .source
                                .contains("https"))
                    {
                        ui.hyperlink(
                            self.current_points[self.point_requesting_action_index]
                                .source
                                .clone(),
                        );
                    } else {
                        ui.label(
                            self.current_points[self.point_requesting_action_index]
                                .source
                                .clone(),
                        );
                    }
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(
                            &mut self.current_points[self.point_requesting_action_index].source,
                        );
                        if ui.button("✅ Add Source").clicked() {
                            update_source(
                                self.project_directory.clone(),
                                self.current_points[self.point_requesting_action_index]
                                    .id
                                    .clone(),
                                self.current_points[self.point_requesting_action_index]
                                    .source
                                    .clone(),
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
