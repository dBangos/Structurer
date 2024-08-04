use crate::save_load::general::load_from_filename;
use crate::save_load::image::delete_image_from_point;
use crate::save_load::link::{get_linked_pairs, link_unlink_title};
use crate::save_load::point::{delete_point, load_points_from_title_id};
use crate::save_load::share::share_unshare_point;
use crate::save_load::source::update_source;
use crate::save_load::title::{delete_title, save_title};
use crate::{left_panel_labels, Point, Structurer};
use eframe::egui::{self, RichText};
use rfd::FileDialog;
impl Structurer {
    pub fn tags_popup(&mut self, ctx: &egui::Context) {
        if self.show_tags_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Tags")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .min_size([600.0, 300.0])
                .max_size([600.0, 600.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Add an existing tag")
                                        .text_style(left_panel_labels()),
                                );
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT)
                                            .with_main_wrap(true),
                                        |ui| {
                                            assert_eq!(
                                                self.all_tags.len(),
                                                self.current_title_tag_bools.len()
                                            );
                                            for (tag_bool, tag) in self
                                                .current_title_tag_bools
                                                .iter_mut()
                                                .zip(self.all_tags.clone())
                                            {
                                                ui.checkbox(tag_bool, tag);
                                            }
                                        },
                                    );
                                });
                            });
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Create a new tag")
                                        .text_style(left_panel_labels()),
                                );

                                ui.text_edit_singleline(&mut self.possible_new_tag);
                                if ui.button("Create").clicked() {
                                    if self.possible_new_tag != String::new() {
                                        self.all_tags.push(self.possible_new_tag.clone());
                                        self.current_title_tag_bools.push(false);
                                        self.possible_new_tag = String::new();
                                    }
                                }
                            });
                        });
                        ui.horizontal(|ui| {
                            if ui.button("✅ Ok").clicked() {
                                self.possible_new_tag = String::new();
                                self.titles[self.current_title_index].tags = Vec::new();
                                for (tag, tag_bool) in self
                                    .all_tags
                                    .clone()
                                    .into_iter()
                                    .zip(self.current_title_tag_bools.clone())
                                {
                                    if tag_bool {
                                        self.titles[self.current_title_index].tags.push(tag);
                                    }
                                }
                                save_title(
                                    self.project_directory.clone(),
                                    self.titles[self.current_title_index].clone(),
                                );
                                self.show_tags_popup = false;
                            }
                            if ui.button("✖ Cancel").clicked() {
                                self.possible_new_tag = String::new();
                                self.show_tags_popup = false;
                            }
                        });
                    });
                });
            self.show_tags_popup &= show_popup;
        }
    }
    pub fn title_edit_popup(&mut self, ctx: &egui::Context) {
        if self.show_title_edit_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Please enter a new name")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.text_edit_singleline(&mut self.titles[self.current_title_index].name);
                        ui.horizontal(|ui| {
                            if ui.button("✅ Ok").clicked() {
                                //Making sure it can't be empty and impossible to click
                                if self.titles[self.current_title_index].name == "".to_string() {
                                    self.titles[self.current_title_index].name =
                                        "New title".to_string();
                                }
                                self.show_title_edit_popup = false;
                            }
                        });
                    });
                });
            self.show_title_edit_popup &= show_popup;
        }
    }

    pub fn point_image_popup(&mut self, ctx: &egui::Context) {
        if self.show_point_image_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
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
                            self.show_point_image_popup = false;
                            //Removing the item from state
                            self.current_points[self.point_image_requesting_popup.0]
                                .images
                                .remove(self.point_image_requesting_popup.1);
                        }
                    });
                });
            self.show_point_image_popup &= show_popup;
        }
    }

    pub fn title_image_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .open(&mut self.show_title_image_popup)
            .show(ctx, |ui| {
                //If there is an image attached, replace the placeholder
                if self.titles[self.current_title_index].image.path.len() > 1 {
                    let file_path = self.titles[self.current_title_index].image.path.clone();
                    let image = egui::Image::new(format!("file://{file_path}"))
                        .fit_to_exact_size([600.0, 600.0].into())
                        .sense(egui::Sense::click());
                    ui.add(image);
                }
                ui.label("Description");
                ui.vertical_centered(|ui| {
                    ui.text_edit_multiline(
                        &mut self.titles[self.current_title_index].image.description,
                    );
                });
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(115.0);
                    if ui.button("🔄 Reset").clicked() {
                        self.titles[self.current_title_index].image.description = String::new();
                        self.titles[self.current_title_index].image.path = String::new();
                    }
                    if ui.button("🖼 Load Image").clicked() {
                        let file = FileDialog::new()
                            .add_filter("image", &["jpeg", "jpg", "png"])
                            .set_directory(self.project_directory.clone())
                            .pick_file();
                        self.titles[self.current_title_index].image.path =
                            file.unwrap_or_default().to_string_lossy().to_string();
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

    pub fn title_delete_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("Confirm Deletion")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
                ui.label("Are you sure you want to permanently delete this title?");
                ui.horizontal(|ui| {
                    ui.add_space(85.0);
                    if ui.button("🗑 Delete").clicked() {
                        delete_title(
                            self.project_directory.clone(),
                            self.titles[self.current_title_index].id.clone(),
                        );
                        //Removing the title from state
                        self.titles.remove(self.current_title_index);
                        //Showing the first title
                        self.current_title_index = 0;
                        if self.titles.len() == 0 {
                            self.title_loaded = false;
                        } else {
                            self.current_points = Vec::new();
                            for new_point_id in
                                self.titles[self.current_title_index].point_ids.clone()
                            {
                                let mut new_point: Point = Point::default();
                                new_point.id = new_point_id.to_string();
                                new_point.content = load_from_filename(
                                    new_point_id.to_string(),
                                    self.project_directory.clone(),
                                );
                                self.current_points.push(new_point);
                            }
                        }
                        self.linked_pairs =
                            get_linked_pairs(self.project_directory.clone(), self.titles.clone());
                        self.show_title_delete_popup = false;
                    }
                    if ui.button("✖ Cancel").clicked() {
                        self.show_title_delete_popup = false;
                    }
                });
            });
    }

    pub fn show_share_point_or_link_title_popup(&mut self, ctx: &egui::Context) {
        assert!(self.current_points.len() >= self.point_requesting_action_index);
        egui::Window::new("")
            .resizable(false)
            .default_pos([700.0, 200.0])
            .min_size([300.0, 300.0])
            .max_size([300.0, 600.0])
            .show(ctx, |ui| {
                if self.show_share_point_popup {
                    ui.label("Share point:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            for (is_shared, checkbox_title) in self
                                .titles_receiving_shared_point
                                .iter_mut()
                                .zip(self.titles.clone())
                            {
                                ui.checkbox(is_shared, checkbox_title.name.clone());
                            }
                        });
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(70.0);
                        if ui.button("✅ Share").clicked() {
                            share_unshare_point(
                                self.project_directory.clone(),
                                self.current_points[self.point_requesting_action_index]
                                    .id
                                    .clone(),
                                self.titles_receiving_shared_point.clone(),
                            );
                            //Adding the point to shared in state, removing from unshared in
                            //state
                            for (title, is_shared) in self
                                .titles
                                .clone()
                                .iter_mut()
                                .zip(self.titles_receiving_shared_point.clone())
                            {
                                if is_shared
                                    && title.point_ids.contains(
                                        &self.current_points[self.point_requesting_action_index]
                                            .id
                                            .clone(),
                                    )
                                {
                                    title.point_ids.push(
                                        self.current_points[self.point_requesting_action_index]
                                            .id
                                            .clone(),
                                    );
                                } else if !is_shared
                                    && title.point_ids.contains(
                                        &self.current_points[self.point_requesting_action_index]
                                            .id
                                            .clone(),
                                    )
                                {
                                    title.point_ids.retain(|x| {
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
                                for title in self.titles.iter_mut() {
                                    title.point_ids.retain(|x| {
                                        *x != self.current_points
                                            [self.point_requesting_action_index]
                                            .id
                                            .clone()
                                    })
                                }
                            }
                            self.current_points = load_points_from_title_id(
                                self.project_directory.clone(),
                                self.titles[self.current_title_index].id.clone(),
                            );
                            self.show_share_point_popup = false;
                        }

                        if ui.button("✖ Cancel").clicked() {
                            self.show_share_point_popup = false;
                        }
                    });
                } else if self.show_link_title_popup {
                    ui.label("Link Title:");
                    ui.vertical(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                //Need to create a local temp to obey the borrow checkers whims
                                let mut local_name_list: Vec<String> = Vec::new();
                                for title in self.titles.clone() {
                                    local_name_list.push(title.name);
                                }
                                for (is_linked, title_name) in self.titles[self.current_title_index]
                                    .links
                                    .iter_mut()
                                    .zip(local_name_list)
                                {
                                    ui.checkbox(is_linked, title_name);
                                }
                            });
                        });
                    });
                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.add_space(75.0);
                        if ui.button("✅ Link").clicked() {
                            link_unlink_title(
                                self.project_directory.clone(),
                                self.current_title_index.clone(),
                                self.titles.clone(),
                            );
                            self.show_link_title_popup = false;
                            self.linked_pairs = get_linked_pairs(
                                self.project_directory.clone(),
                                self.titles.clone(),
                            );
                        }

                        if ui.button("✖ Cancel").clicked() {
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
