use crate::save_load::{
    add_point, add_title, get_point_source, get_title_image, load_from_filename,
    point_is_shared_with, save_title, save_to_filename, title_is_linked_with,
};
use crate::Title;
use crate::{Point, Structurer};
use eframe::egui::{self};
use std::collections::HashMap;
use std::path::PathBuf;
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Set Project Directory").clicked() {
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
            if ui.button("Save").clicked() {
                save_title(self.project_directory.clone(), self.current_title.clone());
                for point in self.current_points.clone() {
                    save_to_filename(self.project_directory.clone(), point.id, point.content);
                }
            }
            if ui.button("Add Point").clicked() {
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
            if ui.button("Add Title").clicked() {
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
            if ui.button("Delete Title").clicked() {
                self.show_title_delete_popup = true;
            }
            if ui.button("Save Page As:").clicked() {
                //
            }
            if ui.button("Link Title").clicked() {
                self.current_title.links = title_is_linked_with(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                );
                self.show_link_title_popup = true;
            }
        });
    }

    //Contains the list of buttons leading to all the titles
    pub fn title_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label("All Titles");
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            for title_id in self.title_order.clone().into_iter() {
                if ui.button(self.titles[&title_id].name.clone()).clicked() {
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
        ui.label("Linked With:");
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
        //let file_path = "/home/sleiren/Coding/structurer/testing/xi.jpg";
        //let image = egui::Image::new(format!("file://{file_path}"))
        //If there is an image attached, replace the placeholder

        ui.horizontal(|ui| {
            if self.current_title.image.path.len() > 1 {
                let file_path = self.current_title.image.path.clone();
                let image = egui::Image::new(format!("file://{file_path}"))
                    .fit_to_exact_size([300.0, 300.0].into())
                    .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.show_image_popup = true;
                }
            } else {
                let image =
                    egui::Image::new(egui::include_image!("../assets/plus-square-icon.png"))
                        .fit_to_exact_size([300.0, 300.0].into())
                        .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.show_image_popup = true;
                }
            }
            ui.text_edit_singleline(&mut self.current_title.name);
        });
    }

    //Contains all the points and their buttons
    pub fn points_layout(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            for (index, point) in self.current_points.iter_mut().enumerate() {
                // Container for elements of each point
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("Delete").clicked() {
                            self.point_requesting_action_index = index;
                            self.show_confirm_delete_popup = true;
                        }
                        if ui.button("Add to:").clicked() {
                            self.titles_receiving_shared_point = point_is_shared_with(
                                self.project_directory.clone(),
                                point.id.clone(),
                            );
                            self.point_requesting_action_index = index;
                            self.show_share_point_popup = true;
                        }
                        if ui.button("Source").clicked() {
                            self.point_requesting_action_index = index;

                            point.source =
                                get_point_source(self.project_directory.clone(), point.id.clone());
                            self.show_source_popup = true;
                        }
                    });

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut point.content),
                    );
                });
            }
        });
    }
}

//Helper function that saves and updates state
//Turned this into a function instead of a method on Structurerto avoid borrow conflicts
pub fn save_old_add_new_points(
    project_directory: PathBuf,
    current_title: Title,
    current_points: Vec<Point>,
    new_title: Title,
) -> (Title, Vec<Point>) {
    //Saving the title of the curent page before switching
    //First checking if the file exists
    let temp_file_path_for_check: PathBuf = [
        project_directory.clone(),
        PathBuf::from(current_title.id.clone() + ".txt"),
    ]
    .iter()
    .collect();
    if temp_file_path_for_check.exists() {
        save_title(project_directory.clone(), current_title.clone());
    }
    let mut return_current_points: Vec<Point> = Vec::new();
    let mut return_title = new_title.clone();
    //Updating the links for the new title_id
    return_title.links = title_is_linked_with(project_directory.clone(), new_title.id.clone());
    return_title.image = get_title_image(project_directory.clone(), new_title.id);
    //Save old points => Remove old points => Add new points
    for point in current_points.clone() {
        save_to_filename(project_directory.clone(), point.id, point.content);
    }
    for new_point_id in new_title.point_ids.into_iter() {
        let mut new_point: Point = Point::default();
        new_point.id = new_point_id.to_string();
        new_point.content = load_from_filename(new_point_id.to_string(), project_directory.clone());
        return_current_points.push(new_point);
    }
    return (return_title, return_current_points);
}
