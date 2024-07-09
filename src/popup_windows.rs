use crate::save_load::{
    delete_point, delete_title, link_unlink_title, load_from_filename, load_points_from_title_id,
    share_unshare_point, update_source,
};
use crate::{Point, Structurer};
use eframe::egui::{self};
impl Structurer {
    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
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
                            if ui.button("Add source").clicked() {
                                update_source(
                                    self.project_directory.clone(),
                                    self.current_points[self.point_requesting_action_index]
                                        .id
                                        .clone(),
                                    self.current_points[self.point_requesting_action_index]
                                        .source
                                        .clone(),
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
                            //Removing the title from state
                            self.titles.remove(&self.current_title.id.clone());
                            self.title_order
                                .retain(|x| *x != self.current_title.id.clone());
                            //Showing the first title
                            self.current_title = self.titles[&self.title_order[0]].clone();
                            self.current_points = Vec::new();
                            for new_point_id in self.titles[&self.title_order[0]].point_ids.clone()
                            {
                                let mut new_point: Point = Point::default();
                                new_point.id = new_point_id.to_string();
                                new_point.content = load_from_filename(
                                    new_point_id.to_string(),
                                    self.project_directory.clone(),
                                );
                                self.current_points.push(new_point);
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
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Confirm Deletion")
                .with_inner_size([200.0, 350.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                egui::CentralPanel::default().show(ctx, |ui| {
                    if self.show_share_point_popup {
                        ui.label("Share point:");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                for (is_shared, checkbox_title_id) in self
                                    .titles_receiving_shared_point
                                    .iter_mut()
                                    .zip(self.title_order.clone())
                                {
                                    ui.checkbox(
                                        is_shared,
                                        self.titles[&checkbox_title_id].name.clone(),
                                    );
                                }
                            });
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Ok").clicked() {
                                share_unshare_point(
                                    self.project_directory.clone(),
                                    self.current_points[self.point_requesting_action_index]
                                        .id
                                        .clone(),
                                    self.titles_receiving_shared_point.clone(),
                                    self.title_order.clone(),
                                );
                                //Adding the point to shared in state, removing from unshared in
                                //state
                                for (title_id, is_shared) in self
                                    .title_order
                                    .clone()
                                    .into_iter()
                                    .zip(self.titles_receiving_shared_point.clone())
                                {
                                    if is_shared
                                        && !self.titles[&title_id].point_ids.contains(
                                            &self.current_points
                                                [self.point_requesting_action_index]
                                                .id
                                                .clone(),
                                        )
                                    {
                                        self.titles.get_mut(&title_id).unwrap().point_ids.push(
                                            self.current_points[self.point_requesting_action_index]
                                                .id
                                                .clone(),
                                        );
                                    } else if !is_shared
                                        && self.titles[&title_id].point_ids.contains(
                                            &self.current_points
                                                [self.point_requesting_action_index]
                                                .id
                                                .clone(),
                                        )
                                    {
                                        self.titles.get_mut(&title_id).unwrap().point_ids.retain(
                                            |x| {
                                                *x != self.current_points
                                                    [self.point_requesting_action_index]
                                                    .id
                                                    .clone()
                                            },
                                        );
                                    }
                                }
                                //If the point is not shared to any titles, delete it
                                if self
                                    .titles_receiving_shared_point
                                    .iter()
                                    .all(|c| *c == false)
                                {
                                    delete_point(
                                        self.project_directory.clone(),
                                        self.current_points[self.point_requesting_action_index]
                                            .id
                                            .clone(),
                                    );
                                    //Removing the point from all titles in state
                                    for title_id in self.title_order.clone() {
                                        self.titles.get_mut(&title_id).unwrap().point_ids.retain(
                                            |x| {
                                                *x != self.current_points
                                                    [self.point_requesting_action_index]
                                                    .id
                                                    .clone()
                                            },
                                        )
                                    }
                                }
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
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    for (is_linked, title_id) in self
                                        .current_title
                                        .links
                                        .iter_mut()
                                        .zip(self.title_order.clone())
                                    {
                                        ui.checkbox(is_linked, self.titles[&title_id].name.clone());
                                    }
                                });
                            });
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Ok").clicked() {
                                link_unlink_title(
                                    self.project_directory.clone(),
                                    self.current_title.clone(),
                                    self.title_order.clone(),
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
        assert!(self.current_points.len() >= self.point_requesting_action_index);
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
                                self.current_points[self.point_requesting_action_index]
                                    .id
                                    .clone(),
                            );
                            //Removing the point from all titles in state
                            for title_id in self.title_order.clone() {
                                self.titles
                                    .get_mut(&title_id)
                                    .unwrap()
                                    .point_ids
                                    .retain(|x| {
                                        *x != self.current_points
                                            [self.point_requesting_action_index]
                                            .id
                                            .clone()
                                    })
                            }
                            //Loading the remaining points
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
