use crate::save_load::{
    add_point, add_title, change_title_name, get_point_source, load_from_filename,
    load_from_library, point_is_shared_with, save_to_filename, title_is_linked_with,
};
use crate::Structurer;
use eframe::egui::{self};
use std::path::PathBuf;
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Set Project Directory").clicked() {
                if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                    self.project_directory = dir_path;
                }
                (self.title_ids, self.titles, self.points_of_title) =
                    load_from_library(self.project_directory.clone());
            }
            if ui.button("Save").clicked() {
                change_title_name(
                    self.project_directory.clone(),
                    self.current_title_id.clone(),
                    self.current_title.clone(),
                );
                for (id, content) in self.current_points.clone().into_iter() {
                    save_to_filename(self.project_directory.clone(), id, content);
                }
            }
            if ui.button("Add Point").clicked() {
                self.current_points.push(add_point(
                    self.project_directory.clone(),
                    self.current_title_id.clone(),
                ));
                (self.title_ids, self.titles, self.points_of_title) =
                    load_from_library(self.project_directory.clone());
            }
            if ui.button("Add Title").clicked() {
                add_title(self.project_directory.clone());
            }
            if ui.button("Delete Title").clicked() {
                self.show_title_delete_popup = true;
            }
            if ui.button("Save Page As:").clicked() {
                self.age += 1;
            }
            if ui.button("Link Title").clicked() {
                self.titles_linked_to_current = title_is_linked_with(
                    self.project_directory.clone(),
                    self.current_title_id.clone(),
                );
                self.show_link_title_popup = true;
            }
        });
    }

    //Contains the list of buttons leading to all the titles
    pub fn title_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label("All Titles");
        ui.vertical(|ui| {
            //Making sure tha data is clean
            let temp_file_path_for_check: PathBuf =
                [self.project_directory.clone(), PathBuf::from("Library.txt")]
                    .iter()
                    .collect();
            if temp_file_path_for_check.exists() {
                (self.title_ids, self.titles, self.points_of_title) =
                    load_from_library(self.project_directory.clone());
            }
            //Binding each title button to loading the corresponding points
            for (title_id, title, t_points) in self
                .title_ids
                .clone()
                .into_iter()
                .zip(self.titles.clone().into_iter())
                .zip(self.points_of_title.clone().into_iter())
                .map(|((x, y), z)| (x, y, z))
            {
                if ui.button(title.clone()).clicked() {
                    self.save_old_add_new_points(title, t_points, title_id);
                }
            }
        });
    }

    //Contians the buttons leading to the currently displayed title's links
    pub fn linked_titles_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label("Linked With:");
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            for (title_id, title, t_points, is_linked) in self
                .title_ids
                .clone()
                .into_iter()
                .zip(self.titles.clone().into_iter())
                .zip(self.points_of_title.clone().into_iter())
                .zip(self.titles_linked_to_current.clone().into_iter())
                .map(|(((x, y), z), a)| (x, y, z, a))
            {
                if is_linked {
                    if ui.button(title.clone()).clicked() {
                        self.save_old_add_new_points(title, t_points, title_id);
                    }
                }
            }
        });
    }

    //Helper function that saves and updates state
    fn save_old_add_new_points(&mut self, title: String, t_points: Vec<String>, title_id: String) {
        //Saving the title of the curent page before switching
        //First checking if the file exists
        let temp_file_path_for_check: PathBuf = [
            self.project_directory.clone(),
            PathBuf::from(self.current_title_id.clone() + ".txt"),
        ]
        .iter()
        .collect();
        if temp_file_path_for_check.exists() {
            change_title_name(
                self.project_directory.clone(),
                self.current_title_id.clone(),
                self.current_title.clone(),
            );
        }
        //Setting the title field
        self.current_title = title.clone();
        self.current_title_id = title_id.clone();
        //Updating the links for the new title_id
        self.titles_linked_to_current = title_is_linked_with(
            self.project_directory.clone(),
            self.current_title_id.clone(),
        );
        //Save old points => Remove old points => Add new points
        for (id, content) in self.current_points.clone().into_iter() {
            save_to_filename(self.project_directory.clone(), id, content);
        }
        self.current_points = Vec::new();
        for new_point in t_points.into_iter() {
            self.current_points.push((
                new_point.to_string(),
                load_from_filename(new_point.to_string(), self.project_directory.clone()),
            ));
        }
    }

    //Contains all the points and their buttons
    pub fn points_layout(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut self.current_title),
                )
            });
            for point in self.current_points.iter_mut() {
                // Container for elements of each point
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("Delete").clicked() {
                            self.point_requesting_deletion = point.0.clone();
                            self.show_confirm_delete_popup = true;
                        }
                        if ui.button("Add to:").clicked() {
                            self.titles_receiving_shared_point = point_is_shared_with(
                                self.project_directory.clone(),
                                point.0.clone(),
                            );
                            self.point_requesting_sharing = point.0.clone();
                            self.show_share_point_popup = true;
                        }
                        if ui.button("Source").clicked() {
                            self.point_requesting_source = point.0.clone();
                            self.point_source = get_point_source(
                                self.project_directory.clone(),
                                self.point_requesting_source.clone(),
                            );
                            self.show_source_popup = true;
                        }
                    });

                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut point.1));
                });
            }
        });
    }
}
