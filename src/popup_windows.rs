use crate::save_load::{
    delete_image_from_point, delete_point, delete_title, link_unlink_title, load_from_filename,
    load_points_from_title_id, share_unshare_point, update_source,
};
use crate::{Point, Structurer};
use eframe::egui::{self};
use rfd::FileDialog;
impl Structurer {
    pub fn point_image_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .open(&mut self.show_point_image_popup)
            .show(ctx, |ui| {
                //If there is an image attached, replace the placeholder
                let file_path = self.current_points[self.point_image_requesting_popup.0].images
                    [self.point_image_requesting_popup.1]
                    .path
                    .clone();
                let image = egui::Image::new(format!("file://{file_path}"))
                    .fit_to_exact_size([600.0, 600.0].into())
                    .sense(egui::Sense::click());
                ui.add(image);
                ui.label("Description");
                ui.horizontal(|ui| {
                    ui.text_edit_multiline(
                        &mut self.current_points[self.point_image_requesting_popup.0].images
                            [self.point_image_requesting_popup.1]
                            .description,
                    );

                    if ui.button("Delete").clicked() {
                        delete_image_from_point(
                            self.project_directory.clone(),
                            self.current_points[self.point_image_requesting_popup.0]
                                .id
                                .clone(),
                            self.current_points[self.point_image_requesting_popup.0].images
                                [self.point_image_requesting_popup.1]
                                .clone(),
                        );
                        ////Removing the item from state
                        //self.current_points[self.point_image_requesting_popup.0]
                        //    .images
                        //    .remove(self.point_image_requesting_popup.1);
                    }
                });
            });
    }

    pub fn title_image_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .open(&mut self.show_title_image_popup)
            .show(ctx, |ui| {
                //If there is an image attached, replace the placeholder
                if self.current_title.image.path.len() > 1 {
                    let file_path = self.current_title.image.path.clone();
                    let image = egui::Image::new(format!("file://{file_path}"))
                        .fit_to_exact_size([600.0, 600.0].into())
                        .sense(egui::Sense::click());
                    ui.add(image);
                }
                ui.label("Description");
                ui.horizontal(|ui| {
                    ui.text_edit_multiline(&mut self.current_title.image.description);
                    if ui.button("Reset").clicked() {
                        self.current_title.image.description = String::new();
                        self.current_title.image.path = String::new();
                    }
                    if ui.button("Load image").clicked() {
                        let file = FileDialog::new()
                            .add_filter("image", &["jpeg", "jpg", "png"])
                            .set_directory(self.project_directory.clone())
                            .pick_file();
                        self.current_title.image.path = file.unwrap().to_string_lossy().to_string();
                    }
                });
            });
    }

    pub fn point_source_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("Confirm Deletion")
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
                            self.show_source_popup = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_source_popup = false;
                        }
                    });
                });
            });
    }

    pub fn title_delete_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("Confirm Deletion")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
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
                        for new_point_id in self.titles[&self.title_order[0]].point_ids.clone() {
                            let mut new_point: Point = Point::default();
                            new_point.id = new_point_id.to_string();
                            new_point.content = load_from_filename(
                                new_point_id.to_string(),
                                self.project_directory.clone(),
                            );
                            self.current_points.push(new_point);
                        }
                        self.show_title_delete_popup = false;
                    }

                    if ui.button("No").clicked() {
                        self.show_title_delete_popup = false;
                    }
                });
            });
    }

    pub fn show_share_point_or_link_title_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
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
                                        &self.current_points[self.point_requesting_action_index]
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
                                        &self.current_points[self.point_requesting_action_index]
                                            .id
                                            .clone(),
                                    )
                                {
                                    self.titles
                                        .get_mut(&title_id)
                                        .unwrap()
                                        .point_ids
                                        .retain(|x| {
                                            *x != self.current_points
                                                [self.point_requesting_action_index]
                                                .id
                                                .clone()
                                        });
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
                            }
                            self.current_points = load_points_from_title_id(
                                self.project_directory.clone(),
                                self.current_title.id.clone(),
                            );
                            self.show_share_point_popup = false;
                        }

                        if ui.button("Cancel").clicked() {
                            self.show_share_point_popup = false;
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
                            self.show_link_title_popup = false;
                        }

                        if ui.button("Cancel").clicked() {
                            self.show_link_title_popup = false;
                        }
                    });
                }
            });
    }

    pub fn confirm_deletion_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("Confirm Deletion")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
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
                                    *x != self.current_points[self.point_requesting_action_index]
                                        .id
                                        .clone()
                                })
                        }
                        //Loading the remaining points
                        self.current_points = load_points_from_title_id(
                            self.project_directory.clone(),
                            self.current_title.id.clone(),
                        );
                        self.show_confirm_delete_popup = false;
                    }

                    if ui.button("No").clicked() {
                        self.show_confirm_delete_popup = false;
                    }
                });
            });
    }
}
