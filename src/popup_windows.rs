use crate::save_load::{
    delete_point, delete_title, link_unlink_title, load_from_filename, load_points_from_title_id,
    share_unshare_point, update_source,
};
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Confirm Deletion")
                .with_inner_size([400.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        //If it's like a link make it a hyperlink
                        if self.point_source != "No source set yet."
                            && (self.point_source.contains("www")
                                || self.point_source.contains("https"))
                        {
                            ui.hyperlink(self.point_source.clone());
                        } else {
                            ui.label(self.point_source.clone());
                        }
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.point_source);
                            if ui.button("Add source").clicked() {
                                update_source(
                                    self.project_directory.clone(),
                                    self.point_requesting_source.clone(),
                                    self.point_source.clone(),
                                );
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        self.show_source_popup = false;
                    }
                });
            },
        );
    }

    pub fn title_delete_popup(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Confirm Deletion")
                .with_inner_size([300.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Are you sure you want to permanently delete this title?");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            delete_title(
                                self.project_directory.clone(),
                                self.current_title.id.clone(),
                            );
                            //Reseting the state and showing the first title
                            self.load_from_library();
                            self.current_title = self.titles[0].clone();
                            self.current_points = Vec::new();
                            for new_point in self.titles[0].point_ids.clone().into_iter() {
                                self.current_points.push((
                                    new_point.to_string(),
                                    load_from_filename(
                                        new_point.to_string(),
                                        self.project_directory.clone(),
                                    ),
                                ));
                            }
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        if ui.button("No").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_title_delete_popup = false;
                    }
                });
            },
        );
    }

    pub fn show_share_point_or_link_title_popup(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Confirm Deletion")
                .with_inner_size([200.0, 300.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.show_share_point_popup {
                        ui.label("Share point:");
                        ui.vertical(|ui| {
                            for (is_shared, checkbox_title) in self
                                .titles_receiving_shared_point
                                .iter_mut()
                                .zip(self.titles.clone())
                            {
                                ui.checkbox(is_shared, checkbox_title.name.clone());
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Ok").clicked() {
                                share_unshare_point(
                                    self.project_directory.clone(),
                                    self.point_requesting_sharing.clone(),
                                    self.titles_receiving_shared_point.clone(),
                                    self.titles.clone(),
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
                                self.load_from_library();
                                self.current_points = load_points_from_title_id(
                                    self.project_directory.clone(),
                                    self.current_title.id.clone(),
                                );
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }

                            if ui.button("Cancel").clicked() {
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    } else if self.show_link_title_popup {
                        ui.label("Link Title:");
                        ui.vertical(|ui| {
                            for (is_linked, title) in
                                self.current_title.links.iter_mut().zip(self.titles.clone())
                            {
                                ui.checkbox(is_linked, title.name);
                            }
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Ok").clicked() {
                                link_unlink_title(
                                    self.project_directory.clone(),
                                    self.current_title.clone(),
                                    self.titles.clone(),
                                );
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
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
                });
            },
        );
    }

    pub fn confirm_deletion_popup(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Confirm Deletion")
                .with_inner_size([300.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Are you sure you want to permanently delete this point?");
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            delete_point(
                                self.project_directory.clone(),
                                self.point_requesting_deletion.clone(),
                            );
                            self.load_from_library();
                            self.current_points = load_points_from_title_id(
                                self.project_directory.clone(),
                                self.current_title.id.clone(),
                            );
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        if ui.button("No").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_confirm_delete_popup = false;
                    }
                });
            },
        );
    }
}
