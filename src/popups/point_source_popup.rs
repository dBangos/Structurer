use crate::{save_load::point::save_point, PopupActive, Structurer};
use eframe::egui::{self};
impl Structurer {
    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::PointSource {
            let mut show_popup = true;
            egui::Window::new("Add source to point")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        //If it's like a link make it a hyperlink
                        if self.points[&self.point_requesting_action_id].source
                            != "No source set yet."
                            && (self.points[&self.point_requesting_action_id]
                                .source
                                .contains("www")
                                || self.points[&self.point_requesting_action_id]
                                    .source
                                    .contains("https"))
                        {
                            ui.hyperlink(
                                self.points[&self.point_requesting_action_id].source.clone(),
                            );
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
                                save_point(
                                    self.project_directory.clone(),
                                    self.points[&self.point_requesting_action_id].clone(),
                                );
                                self.popup_active = PopupActive::Empty;
                            }
                            if ui.button("✖ Cancel").clicked() {
                                self.popup_active = PopupActive::Empty;
                            }
                        });
                    });
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }
}
