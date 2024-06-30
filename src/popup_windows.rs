use crate::save_load::{
    delete_point, delete_title, link_unlink_title, load_from_filename, load_from_library,
    load_points_from_title_id, share_unshare_point,
};
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn title_delete_popup(&mut self, ui: &mut egui::Ui) {
        ui.label("Are you sure you want to permanently delete this title?");
        ui.horizontal(|ui| {
            if ui.button("Yes").clicked() {
                delete_title(
                    self.project_directory.clone(),
                    self.current_title_id.clone(),
                );
                //Reseting the state and showing the first title
                (self.title_ids, self.titles, self.points_of_title) =
                    load_from_library(self.project_directory.clone());
                (self.current_title_id, self.current_title) =
                    (self.title_ids[0].clone(), self.titles[0].clone());
                self.current_points = Vec::new();
                for new_point in self.points_of_title[0].clone().into_iter() {
                    self.current_points.push((
                        new_point.to_string(),
                        load_from_filename(new_point.to_string(), self.project_directory.clone()),
                    ));
                }
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                ui.ctx().request_repaint();
            }

            if ui.button("No").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        if ui.ctx().input(|i| i.viewport().close_requested()) {
            // Tell parent viewport that we should not show next frame:
            self.show_title_delete_popup = false;
        }
    }

    pub fn show_share_point_or_link_title_popup(&mut self, ui: &mut egui::Ui) {
        if self.show_share_point_popup {
            ui.label("Share point:");
            ui.vertical(|ui| {
                for (is_shared, title) in self
                    .titles_receiving_shared_point
                    .iter_mut()
                    .zip(self.titles.clone())
                {
                    ui.checkbox(is_shared, title.clone());
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Ok").clicked() {
                    share_unshare_point(
                        self.project_directory.clone(),
                        self.point_requesting_sharing.clone(),
                        self.titles_receiving_shared_point.clone(),
                        self.title_ids.clone(),
                    );
                    //If the point is not shared to any titles, delete it
                    if self
                        .titles_receiving_shared_point
                        .iter()
                        .all(|c| *c == false)
                    {
                        delete_point(
                            self.project_directory.clone(),
                            self.point_requesting_sharing.clone(),
                        );
                    }
                    (self.title_ids, self.titles, self.points_of_title) =
                        load_from_library(self.project_directory.clone());
                    self.current_points = load_points_from_title_id(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                    );
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    ui.ctx().request_repaint();
                }

                if ui.button("Cancel").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        } else if self.show_link_title_popup {
            ui.label("Link Title:");
            ui.vertical(|ui| {
                for (is_linked, title) in self
                    .titles_linked_to_current
                    .iter_mut()
                    .zip(self.titles.clone())
                {
                    ui.checkbox(is_linked, title.clone());
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Ok").clicked() {
                    link_unlink_title(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                        self.titles_linked_to_current.clone(),
                        self.title_ids.clone(),
                    );
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    ui.ctx().request_repaint();
                }

                if ui.button("Cancel").clicked() {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        }
        if ui.ctx().input(|i| i.viewport().close_requested()) {
            // Tell parent viewport that we should not show next frame:
            self.show_link_title_popup = false;
            self.show_share_point_popup = false;
        }
    }

    pub fn confirm_deletion_popup(&mut self, ui: &mut egui::Ui) {
        ui.label("Are you sure you want to permanently delete this point?");
        ui.horizontal(|ui| {
            if ui.button("Yes").clicked() {
                delete_point(
                    self.project_directory.clone(),
                    self.point_requesting_deletion.clone(),
                );
                (self.title_ids, self.titles, self.points_of_title) =
                    load_from_library(self.project_directory.clone());
                self.current_points = load_points_from_title_id(
                    self.project_directory.clone(),
                    self.current_title_id.clone(),
                );
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                ui.ctx().request_repaint();
            }

            if ui.button("No").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
        if ui.ctx().input(|i| i.viewport().close_requested()) {
            // Tell parent viewport that we should not show next frame:
            self.show_confirm_delete_popup = false;
        }
    }
}
