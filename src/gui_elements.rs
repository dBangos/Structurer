use std::usize;

use crate::save_load::general::save_old_add_new_points;
use crate::save_load::image::add_image_to_point;
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::{add_point, save_point};
use crate::save_load::share::point_is_shared_with;
use crate::save_load::source::get_point_source;
use crate::save_load::title::{add_title, save_title};
use crate::{left_panel_labels, title_style, Structurer};
use crate::{ImageStruct, Title};
use eframe::egui::{self, Button, RichText, TextWrapMode};
use eframe::emath::Numeric;
use egui::Vec2;
use rfd::FileDialog;
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("🗀 Set Project Directory").clicked() {
                if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                    //Resetting state in case old values don't get overwritten, in the absence of a
                    //previous library
                    self.titles = Vec::new();
                    self.current_title_index = 0;
                    self.view_scale = 1.0;
                    self.project_directory = dir_path;
                    let _ = self.save_to_config();
                    self.create_library_files();
                }
                self.load_from_library();
            }
            if ui.button("💾 Save").clicked() {
                if let Some(()) = save_title(
                    self.project_directory.clone(),
                    self.titles[self.current_title_index].clone(),
                ) {
                    for point in self.current_points.clone() {
                        save_point(self.project_directory.clone(), point);
                    }
                    //Saving here so save button updates the point_text_size on the json file
                    let _ = self.save_to_config();
                }
            }
            ui.separator();
            //if ui.button("Save Page As:").clicked() {
            //    //
            //}
            if ui.button("➕ Add Title").clicked() {
                //Create new title files
                let new_title_id = add_title(self.project_directory.clone());
                //Add new title to state
                let mut temp_title: Title = Title::default();
                temp_title.id = new_title_id.clone();
                temp_title.name = "New title".to_string();
                //Add point to the new title
                let temp_point = add_point(self.project_directory.clone(), temp_title.id.clone());
                if let Some(p) = temp_point {
                    self.current_points.push(p.clone());
                    //Add new point to state
                    temp_title.point_ids.push(p.id);
                }
                //Switch focus to the new title page
                self.titles.push(temp_title);
                if self.title_loaded {
                    self.current_points = save_old_add_new_points(
                        self.project_directory.clone(),
                        self.titles[self.current_title_index].clone(),
                        self.current_points.clone(),
                        self.titles[self.titles.len() - 1].clone(),
                    );
                } else {
                    self.current_points = save_old_add_new_points(
                        self.project_directory.clone(),
                        Title::default(),
                        self.current_points.clone(),
                        self.titles[0].clone(),
                    );
                }
                self.current_title_index = self.titles.len() - 1;
            }
            if ui.button("↔ Link Title").clicked() {
                self.titles[self.current_title_index].links = title_is_linked_with(
                    self.project_directory.clone(),
                    self.titles[self.current_title_index].id.clone(),
                );
                self.show_link_title_popup = true;
            }
            if ui.button("🗑 Delete Title").clicked() {
                self.show_title_delete_popup = true;
            }
            ui.separator();
            if ui.button("+ Add Point").clicked() {
                if let Some(p) = add_point(
                    self.project_directory.clone(),
                    self.titles[self.current_title_index].id.clone(),
                ) {
                    self.current_points.push(p.clone());
                    self.titles[self.current_title_index].point_ids.push(p.id);
                }
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
            for index in 0..self.titles.len() {
                //If the string is too long shorten it and add ...
                let button_name: String;
                if (self.titles[index].name.clone().len() * 8) as f32 > ui.available_size().x.into()
                {
                    let char_count = ((ui.available_size().x / 8.0) - 3.0) as usize;
                    button_name = self.titles[index].name.clone()[..char_count].to_string() + "...";
                } else {
                    button_name = self.titles[index].name.clone();
                }

                //Binding each title button to loading the corresponding points
                if ui
                    .add(Button::new(button_name).wrap_mode(TextWrapMode::Extend))
                    .clicked()
                {
                    if self.title_loaded == false {
                        self.title_loaded = true;
                        self.current_title_index = index;
                    }

                    self.current_points = save_old_add_new_points(
                        self.project_directory.clone(),
                        self.titles[self.current_title_index].clone(),
                        self.current_points.clone(),
                        self.titles[index].clone(),
                    );
                    self.current_title_index = index;
                    self.titles[index].links = title_is_linked_with(
                        self.project_directory.clone(),
                        self.titles[index].id.clone(),
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
            if self.title_loaded {
                for (index, is_linked) in self.titles[self.current_title_index]
                    .links
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    if is_linked {
                        //If the string is too long shorten it and add ...
                        let button_name: String;
                        if (self.titles[index].name.clone().len() * 8) as f32
                            > ui.available_size().x.into()
                        {
                            let char_count = ((ui.available_size().x / 8.0) - 3.0) as usize;
                            button_name =
                                self.titles[index].name.clone()[..char_count].to_string() + "...";
                        } else {
                            button_name = self.titles[index].name.clone();
                        }

                        //Binding each title button to loading the corresponding points
                        if ui
                            .add(Button::new(button_name).wrap_mode(TextWrapMode::Extend))
                            .clicked()
                        {
                            self.current_points = save_old_add_new_points(
                                self.project_directory.clone(),
                                self.titles[self.current_title_index].clone(),
                                self.current_points.clone(),
                                self.titles[index].clone(),
                            );
                            self.current_title_index = index;
                            self.titles[index].links = title_is_linked_with(
                                self.project_directory.clone(),
                                self.titles[index].id.clone(),
                            );
                        }
                    }
                }
            }
        });
    }

    //Contains the title image and fields
    pub fn title_layout(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if self.titles[self.current_title_index].image.path.len() > 1 {
                let file_path = self.titles[self.current_title_index].image.path.clone();
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
                    RichText::new(self.titles[self.current_title_index].name.clone())
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
                ui.add_space(5.0);
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
                            if let Some(f) = FileDialog::new()
                                .add_filter("image", &["jpeg", "jpg", "png"])
                                .set_directory(self.project_directory.clone())
                                .pick_file()
                            {
                                let mut new_image: ImageStruct = ImageStruct::default();
                                new_image.path = f.to_string_lossy().to_string();
                                point.images.push(new_image.clone());
                                add_image_to_point(
                                    self.project_directory.clone(),
                                    point.id.clone(),
                                    new_image,
                                );
                            }
                        }
                    });
                    ui.vertical(|ui| {
                        ui.style_mut().spacing.item_spacing = Vec2::new(1.0, 1.0);
                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::LEFT).with_main_wrap(true),
                            |ui| {
                                for (image_index, image) in
                                    point.images.clone().into_iter().enumerate()
                                {
                                    let file_path = image.path.clone();
                                    let curr_image =
                                        egui::Image::new(format!("file://{file_path}"))
                                            .fit_to_original_size(2.0)
                                            .max_height(70.0)
                                            .sense(egui::Sense::click());

                                    if ui.add(curr_image).clicked() {
                                        self.point_image_requesting_popup = (index, image_index);
                                        self.show_point_image_popup = true;
                                    }
                                }
                            },
                        );

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
