use crate::save_load::general::save_old_add_new_points;
use crate::save_load::image::add_image_to_point;
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::{add_point, save_point};
use crate::save_load::share::point_is_shared_with;
use crate::save_load::source::get_point_source;
use crate::save_load::title::{add_title, save_title};
use crate::{left_panel_labels, title_style, Structurer};
use crate::{ImageStruct, Point, Title};
use chrono::{Datelike, Timelike};
use eframe::egui::{self, Button, RichText, TextWrapMode};
use egui::{Id, Vec2};
use egui_dnd::{dnd, DragDropItem};
use rfd::FileDialog;

impl DragDropItem for &mut Title {
    fn id(&self) -> Id {
        Id::new(&self.id)
    }
}

impl DragDropItem for &mut Point {
    fn id(&self) -> Id {
        Id::new(&self.id)
    }
}
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
                    self.view_scale = 0.85;
                    self.project_directory = dir_path;
                    self.current_points = Vec::new();
                    self.title_loaded = false;
                    let _ = self.save_to_config();
                    self.create_library_files();
                    self.load_from_library();
                    ui.ctx().forget_all_images();
                }
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
            ui.separator();
            if ui.button("📑 Tags").clicked() {
                self.show_tags_popup = true;
            }
            ui.separator();
            if ui.button("📅 Timeline").clicked() {
                //Update the points to make sure the points are up to date
                self.get_all_points();
                //Filter and sort the points by date and time
                self.all_points.retain(|x| x.date != None);
                self.all_points
                    .sort_by(|a, b| a.date.cmp(&b.date).then(a.time.cmp(&b.time)));
                self.show_timeline_popup = true;
            }
            ui.separator();
            ui.text_edit_singleline(&mut self.searching_string);
            //User can erase the string to end the search
            if self.searching_string == "" {
                self.search_active = false
            }
            if ui.button("Search").clicked() {
                if self.searching_string != "" {
                    self.get_all_points();
                    self.all_points
                        .retain(|x| x.content.contains(&self.searching_string));
                    self.search_active = true;
                }
            }
        });
        //If filtering based on tags
        if self.tags_actively_filtering.iter().any(|&x| x == true) {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Only showing titles with tags:");
                self.tags_in_filter = Vec::new();
                for (tag_bool, tag) in self
                    .tags_actively_filtering
                    .iter_mut()
                    .zip(self.all_tags.clone())
                {
                    if *tag_bool {
                        ui.checkbox(tag_bool, tag.clone());
                        self.tags_in_filter.push(tag);
                    }
                }
                if ui.button("↺ Reset").clicked() {
                    self.tags_actively_filtering = vec![false; self.all_tags.len()];
                    self.tags_in_filter = Vec::new();
                }
            });
            ui.add_space(2.0);
        } else if self.search_active {
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Searching for points containing {}",
                    self.searching_string
                ));
                if ui.button("↺ Reset").clicked() {
                    self.search_active = false;
                    self.searching_string = String::new();
                }
            });
            ui.add_space(2.0);
        }
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
            let tag_filter = self.tags_actively_filtering.iter().any(|&x| x == true);
            let mut index: usize = 0;
            let mut index_of_button_clicked: Option<usize> = None;
            //Drag and drop functionality
            let response =
                dnd(ui, "dnd").show(self.titles.iter_mut(), |ui, title, handle, _state| {
                    //If the filter is active and the title has the tags
                    if tag_filter
                        && self
                            .tags_in_filter
                            .iter()
                            .all(|item| title.tags.contains(item))
                    {
                        handle.ui(ui, |ui| {
                            if ui
                                .add(
                                    Button::new(title.name.clone())
                                        .wrap_mode(TextWrapMode::Truncate),
                                )
                                .clicked()
                            {
                                if self.title_loaded == false {
                                    self.title_loaded = true;
                                    self.current_title_index = index;
                                }
                                index_of_button_clicked = Some(index);
                            }
                            index += 1;
                        });
                    // If the filter isn't active
                    } else if !tag_filter {
                        handle.ui(ui, |ui| {
                            if ui
                                .add(
                                    Button::new(title.name.clone())
                                        .wrap_mode(TextWrapMode::Truncate),
                                )
                                .clicked()
                            {
                                if self.title_loaded == false {
                                    self.title_loaded = true;
                                    self.current_title_index = index;
                                }
                                index_of_button_clicked = Some(index);
                            }
                            index += 1;
                        });
                    }
                });
            if let Some(update) = response.final_update() {
                self.change_title_position(update.from, update.to);
            }
            if let Some(idx) = index_of_button_clicked {
                self.change_title(idx);
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
                        //Binding each title button to loading the corresponding points
                        if ui
                            .add(
                                Button::new(self.titles[index].name.clone())
                                    .wrap_mode(TextWrapMode::Truncate),
                            )
                            .clicked()
                        {
                            self.change_title(index);
                        }
                    }
                }
            }
        });
    }

    //Contains the title image and fields
    pub fn title_layout(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            //If there is an image show it, else show the placeholder
            if self.titles[self.current_title_index].image.path.len() > 1 {
                let file_path = self.titles[self.current_title_index].image.path.clone();
                let image = egui::Image::new(format!("file://{file_path}"))
                    .fit_to_original_size(2.0)
                    .max_height(200.0)
                    .max_width(500.0)
                    .maintain_aspect_ratio(true)
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
            ui.add_space(10.0);
            ui.vertical(|ui| {
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
                ui.horizontal(|ui| {
                    //Add tag buttons
                    for tag in self.titles[self.current_title_index].tags.clone() {
                        //On click filter by tag
                        if ui.button(tag.clone()).clicked() {
                            //If the last checkbox got unchecked empty the string vector
                            if self.tags_actively_filtering.iter().all(|&x| x == false) {
                                self.tags_in_filter = Vec::new();
                            }
                            //If not already filtering with this tag, only then filter with it
                            if !self.tags_in_filter.contains(&tag) {
                                self.tags_in_filter.push(tag.clone());
                                assert_eq!(self.all_tags.len(), self.tags_actively_filtering.len());
                                if let Some(index) = self.all_tags.iter().position(|x| *x == tag) {
                                    self.tags_actively_filtering[index] = true;
                                }
                            }
                        }
                    }
                    let mut add_tag_label: String = "Add Tag".to_string();
                    if self.titles[self.current_title_index].tags.len() > 0 {
                        add_tag_label = "+".to_string();
                    }
                    if ui.button(add_tag_label).clicked() {
                        self.current_title_tag_bools = Vec::new();
                        for tag in self.all_tags.clone() {
                            if self.titles[self.current_title_index].tags.contains(&tag) {
                                self.current_title_tag_bools.push(true);
                            } else {
                                self.current_title_tag_bools.push(false);
                            }
                        }
                        self.show_add_tags_popup = true;
                    }
                });
            });
        });
    }

    //Contains all the points and their buttons
    pub fn points_layout(&mut self, ui: &mut egui::Ui) {
        let mut index: usize = 0;
        ui.vertical(|ui| {
            let response = dnd(ui, "dnd2").show(
                self.current_points.iter_mut(),
                |ui, point, handle, _state| {
                    // Container for elements of each point
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                handle.ui(ui, |ui| {
                                    ui.label("↕");
                                });
                                ui.menu_button("➕ Add..", |ui| {
                                    if ui.button("🖼 Images").clicked() {
                                        ui.close_menu();
                                        if let Some(files) = FileDialog::new()
                                            .add_filter("image", &["jpeg", "jpg", "png", "webp"])
                                            .set_directory(self.project_directory.clone())
                                            .pick_files()
                                        {
                                            for file in files {
                                                let mut new_image: ImageStruct =
                                                    ImageStruct::default();
                                                new_image.path = file.to_string_lossy().to_string();
                                                point.images.push(new_image.clone());
                                                add_image_to_point(
                                                    self.project_directory.clone(),
                                                    point.id.clone(),
                                                    new_image,
                                                );
                                            }
                                        }
                                    }
                                    if ui.button("📆 Date").clicked() {
                                        self.point_requesting_action_index = index;
                                        if let Some(date) = point.date {
                                            self.point_popup_fields.0 = date.year();
                                            self.point_popup_fields.1 = date.month();
                                            self.point_popup_fields.2 = date.day();
                                        }
                                        if let Some(time) = point.time {
                                            self.point_popup_fields.3 = time.hour();
                                            self.point_popup_fields.4 = time.minute();
                                            self.point_popup_fields.5 = time.second();
                                        }
                                        self.show_point_datetime_popup = true;
                                    }
                                });
                            });
                            if ui.button("🗑 Delete").clicked() {
                                self.point_requesting_action_index = index;
                                self.show_confirm_delete_popup = true;
                            }
                            if ui.button("🔀 Share").clicked() {
                                self.titles_receiving_shared_point = point_is_shared_with(
                                    self.project_directory.clone(),
                                    point.id.clone(),
                                );
                                self.point_requesting_action_index = index;
                                self.show_share_point_popup = true;
                            }
                            if ui.button("ℹ Source").clicked() {
                                self.point_requesting_action_index = index;

                                point.source = get_point_source(
                                    self.project_directory.clone(),
                                    point.id.clone(),
                                );
                                self.show_source_popup = true;
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
                                            self.point_image_requesting_popup =
                                                (index, image_index);
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
                    index += 1;
                },
            );
            if let Some(update) = response.final_update() {
                self.change_point_position(update.from, update.to);
            }
        });
    }
    pub fn search_layout(&mut self, ui: &mut egui::Ui) {
        for point in self.all_points.clone() {
            ui.vertical(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(1.0, 1.0);
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::LEFT).with_main_wrap(true),
                    |ui| {
                        for image in point.images.clone() {
                            let file_path = image.path.clone();
                            let curr_image = egui::Image::new(format!("file://{file_path}"))
                                .fit_to_original_size(2.0)
                                .max_height(70.0)
                                .sense(egui::Sense::click());
                            ui.add(curr_image);
                        }
                    },
                );

                ui.label(point.content);
            });
            ui.separator();
        }
    }
}
