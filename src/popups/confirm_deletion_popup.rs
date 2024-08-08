use crate::save_load::point::{delete_point, load_points_from_title_id};
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn confirm_deletion_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("Confirm Deletion")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
                ui.label("Are you sure you want to permanently delete this point?");
                ui.horizontal(|ui| {
                    ui.add_space(85.0);
                    if ui.button("🗑 Delete").clicked() {
                        delete_point(
                            self.project_directory.clone(),
                            self.current_points[self.point_requesting_action_index]
                                .id
                                .clone(),
                        );
                        //Removing the point from all titles in state
                        for title in self.titles.iter_mut() {
                            title.point_ids.retain(|x| {
                                *x != self.current_points[self.point_requesting_action_index]
                                    .id
                                    .clone()
                            })
                        }
                        //Loading the remaining points
                        self.current_points = load_points_from_title_id(
                            self.project_directory.clone(),
                            self.titles[self.current_title_index].id.clone(),
                        );
                        self.show_confirm_delete_popup = false;
                    }

                    if ui.button("✖ Cancel").clicked() {
                        self.show_confirm_delete_popup = false;
                    }
                });
            });
    }
}
