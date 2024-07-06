use crate::save_load::{
    add_point, add_title, change_title_name, get_point_source, load_from_filename,
    point_is_shared_with, save_to_filename, title_is_linked_with,
};
use crate::Title;
use crate::{Point, Structurer};
use eframe::egui::{self};
use std::path::PathBuf;
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Set Project Directory").clicked() {
                if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                    self.project_directory = dir_path;
                    let _ = self.save_to_config();
                }
                self.load_from_library();
            }
            if ui.button("Save").clicked() {
                change_title_name(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                    self.current_title.name.clone(),
                );
                for point in self.current_points.clone() {
                    save_to_filename(self.project_directory.clone(), point.id, point.content);
                }
                self.load_from_library();
            }
            if ui.button("Add Point").clicked() {
                self.current_points.push(add_point(
                    self.project_directory.clone(),
                    self.current_title.id.clone(),
                ));
                self.load_from_library();
            }
            if ui.button("Add Title").clicked() {
                add_title(self.project_directory.clone());
                self.load_from_library();
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
            for title in self.titles.clone().into_iter() {
                if ui.button(title.name.clone()).clicked() {
                    self.save_old_add_new_points(title);
                }
            }
        });
    }

    //Contians the buttons leading to the currently displayed title's links
    pub fn linked_titles_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label("Linked With:");
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            for (title, is_linked) in self
                .titles
                .clone()
                .into_iter()
                .zip(self.current_title.links.clone())
            {
                if is_linked {
                    if ui.button(title.name.clone()).clicked() {
                        self.save_old_add_new_points(title);
                    }
                }
            }
        });
    }

    //Helper function that saves and updates state
    pub fn save_old_add_new_points(&mut self, title: Title) {
        //Saving the title of the curent page before switching
        //First checking if the file exists
        let temp_file_path_for_check: PathBuf = [
            self.project_directory.clone(),
            PathBuf::from(self.current_title.id.clone() + ".txt"),
        ]
        .iter()
        .collect();
        if temp_file_path_for_check.exists() {
            change_title_name(
                self.project_directory.clone(),
                self.current_title.id.clone(),
                self.current_title.name.clone(),
            );
        }
        self.current_title = title.clone();
        //Updating the links for the new title_id
        self.current_title.links = title_is_linked_with(
            self.project_directory.clone(),
            self.current_title.id.clone(),
        );
        //Save old points => Remove old points => Add new points
        for point in self.current_points.clone() {
            save_to_filename(self.project_directory.clone(), point.id, point.content);
        }
        self.current_points = Vec::new();
        for new_point_id in title.point_ids.into_iter() {
            let mut new_point: Point = Point::default();
            new_point.id = new_point_id.to_string();
            new_point.content =
                load_from_filename(new_point_id.to_string(), self.project_directory.clone());
            self.current_points.push(new_point);
        }
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
