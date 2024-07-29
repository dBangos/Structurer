use crate::save_load::general::{save_old_add_new_points, save_to_filename};
use crate::save_load::image::add_image_to_point;
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::add_point;
use crate::save_load::share::point_is_shared_with;
use crate::save_load::source::get_point_source;
use crate::save_load::title::{add_title, save_title};
use crate::{left_panel_labels, title_style, Structurer};
use crate::{ImageStruct, Title};
use eframe::egui::{self, RichText};
use rfd::FileDialog;
use std::collections::HashMap;
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🗀 Set Project Directory").clicked() {
                if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                    //Resetting state in case old values don't get overwritten, in the absence of a
                    //previous library
                    self.titles = HashMap::new();
                    self.title_order = Vec::new();
                    self.current_points = Vec::new();
                    self.current_title = Title::default();
                    self.view_scale = 1.0;
                    self.project_directory = dir_path;
                    let _ = self.save_to_config();
                    self.create_library_links();
                }
                self.load_from_library();
            }
            if ui.button("💾 Save").clicked() {
                save_title(self.project_directory.clone(), self.current_title.clone());
                for point in self.current_points.clone() {
                    save_to_filename(self.project_directory.clone(), point.id, point.content);
                }
                *self.titles.get_mut(&self.current_title.id).unwrap() = self.current_title.clone();
                //Saving here so save button updates the point_text_size on the json file
                let _ = self.save_to_config();
            }
            //if ui.button("Save Page As:").clicked() {
            //    //
            //}
            if ui.button("➕ Add Title").clicked() {
                //Create new title files
                let new_title_id = add_title(self.project_directory.clone());
                //Add new title to state
                self.title_order.push(new_title_id.clone());
                let mut temp_title: Title = Title::default();
                temp_title.id = new_title_id.clone();
                temp_title.name = "New title".to_string();
                self.titles
                    .insert(temp_title.id.clone(), temp_title.clone());
                //Switch focus to the new title page
                (self.current_title, self.current_points) = save_old_add_new_points(
                    self.project_directory.clone(),
                    self.current_title.clone(),
                    self.current_points.clone(),
                    self.titles[self
                        .title_order
                        .last()
                        .expect("Error while accesing last title")]
                    .clone(),
                );
                //Add point to the new title
                self.current_points.push(add_point(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                ));
                //Add new point to state
                self.titles
                    .get_mut(&new_title_id)
                    .unwrap()
                    .point_ids
                    .push(self.current_points[0].id.clone());
            }
            if ui.button("↔ Link Title").clicked() {
                self.current_title.links = title_is_linked_with(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                );
                self.show_link_title_popup = true;
            }
            if ui.button("🗑 Delete Title").clicked() {
                self.show_title_delete_popup = true;
            }
            if ui.button("+ Add Point").clicked() {
                let temp_point = add_point(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                );
                self.current_points.push(temp_point.clone());
                self.titles
                    .get_mut(&self.current_title.id)
                    .unwrap()
                    .point_ids
                    .push(temp_point.id);
            }
        });
    }

    //Contains the list of buttons leading to all the titles
    pub fn title_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("All Titles")
                .text_style(left_panel_labels())
                .strong(),
        );
        ui.separator();
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            for title_id in self.title_order.clone().into_iter() {
                if ui.button(self.titles[&title_id].name.clone()).clicked() {
                    if self.current_title.id.len() > 0 {
                        *self.titles.get_mut(&self.current_title.id).unwrap() =
                            self.current_title.clone();
                    }
                    (self.current_title, self.current_points) = save_old_add_new_points(
                        self.project_directory.clone(),
                        self.current_title.clone(),
                        self.current_points.clone(),
                        self.titles[&title_id].clone(),
                    );
                }
            }
        });
    }

    //Contians the buttons leading to the currently displayed title's links
    pub fn linked_titles_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Linked Titles")
                .text_style(left_panel_labels())
                .strong(),
        );
        ui.separator();
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            for (title_id, is_linked) in self
                .title_order
                .clone()
                .into_iter()
                .zip(self.current_title.links.clone())
            {
                if is_linked {
                    if ui.button(self.titles[&title_id].name.clone()).clicked() {
                        if self.current_title.id.len() > 0 {
                            *self.titles.get_mut(&self.current_title.id).unwrap() =
                                self.current_title.clone();
                        }
                        (self.current_title, self.current_points) = save_old_add_new_points(
                            self.project_directory.clone(),
                            self.current_title.clone(),
                            self.current_points.clone(),
                            self.titles[&title_id].clone(),
                        );
                    }
                }
            }
        });
    }

    //Contains the title image and fields
    pub fn title_layout(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.current_title.image.path.len() > 1 {
                let file_path = self.current_title.image.path.clone();
                let image = egui::Image::new(format!("file://{file_path}"))
                    .fit_to_exact_size([220.0, 220.0].into())
                    .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.show_title_image_popup = true;
                }
            } else {
                let image =
                    egui::Image::new(egui::include_image!("../assets/plus-square-icon.png"))
                        .fit_to_exact_size([220.0, 220.0].into())
                        .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.show_title_image_popup = true;
                }
            }
            if ui
                .label(
                    RichText::new(self.current_title.name.clone())
                        .text_style(title_style())
                        .strong(),
                )
                .clicked()
            {
                self.show_title_edit_popup = true;
            }

            //ui.text_edit_singleline(&mut self.current_title.name);
        });
    }

    //Contains all the points and their buttons
    pub fn points_layout(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            for (index, point) in self.current_points.iter_mut().enumerate() {
                // Container for elements of each point
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("🗑 Delete").clicked() {
                            self.point_requesting_action_index = index;
                            self.show_confirm_delete_popup = true;
                        }
                        if ui.button("➕ Share").clicked() {
                            self.titles_receiving_shared_point = point_is_shared_with(
                                self.project_directory.clone(),
                                point.id.clone(),
                            );
                            self.point_requesting_action_index = index;
                            self.show_share_point_popup = true;
                        }
                        if ui.button("ℹ Source").clicked() {
                            self.point_requesting_action_index = index;

                            point.source =
                                get_point_source(self.project_directory.clone(), point.id.clone());
                            self.show_source_popup = true;
                        }
                        if ui.button("🖼 Add Image").clicked() {
                            let file = FileDialog::new()
                                .add_filter("image", &["jpeg", "jpg", "png"])
                                .set_directory(self.project_directory.clone())
                                .pick_file();
                            let mut new_image: ImageStruct = ImageStruct::default();
                            new_image.path = file.unwrap().to_string_lossy().to_string();
                            point.images.push(new_image.clone());
                            add_image_to_point(
                                self.project_directory.clone(),
                                point.id.clone(),
                                new_image,
                            );
                        }
                    });
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            for (image_index, image) in point.images.clone().into_iter().enumerate()
                            {
                                let file_path = image.path.clone();
                                let curr_image = egui::Image::new(format!("file://{file_path}"))
                                    .fit_to_original_size(2.0)
                                    .max_height(70.0)
                                    .sense(egui::Sense::click());
                                if ui.add(curr_image).clicked() {
                                    self.point_image_requesting_popup = (index, image_index);
                                    self.show_point_image_popup = true;
                                }
                            }
                        });

                        ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::multiline(&mut point.content),
                        );
                    });
                });
            }
        });
    }
}
