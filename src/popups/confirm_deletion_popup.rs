use crate::Structurer;
use crate::{save_load::point::delete_point, PopupActive};
use eframe::egui::{self};
impl Structurer {
    pub fn confirm_deletion_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::ConfirmPointDeletion {
            let mut show_popup = true;
            egui::Window::new("Confirm Deletion")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.label("Are you sure you want to permanently delete this point?");
                    ui.horizontal(|ui| {
                        ui.add_space(85.0);
                        if ui.button("🗑 Delete").clicked() {
                            delete_point(
                                self.project_directory.clone(),
                                self.point_requesting_action_id.clone(),
                            );
                            //Removing the point from all titles in state
                            for title in self.titles.iter_mut() {
                                title
                                    .point_ids
                                    .retain(|x| *x != self.point_requesting_action_id.clone())
                            }
                            //Loading the remaining points
                            self.current_point_ids =
                                self.titles[self.current_title_index].point_ids.clone();
                            self.popup_active = PopupActive::Empty;
                        }

                        if ui.button("✖ Cancel").clicked() {
                            self.popup_active = PopupActive::Empty;
                        }
                    });
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }
}
