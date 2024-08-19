use crate::save_load::general::save_to_filename;
use crate::PopupActive;
use crate::Structurer;
use crate::Title;

impl Structurer {
    pub fn export_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::Export {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Export")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .fixed_size([500.0, 500.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        if ui.button("Select export directory").clicked() {
                            if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                                self.export_directory = Some(dir);
                            }
                        }
                        ui.horizontal(|ui| {
                            ui.label("Exporting to: ");
                            match self.export_directory.clone() {
                                None => ui.label("None"),
                                Some(dir) => ui.label(dir.to_str().unwrap_or("None")),
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Export:");
                            if ui
                                .add(egui::SelectableLabel::new(
                                    self.export_bools[0] == false,
                                    "Current Title",
                                ))
                                .clicked()
                            {
                                self.export_bools[0] = false;
                            }
                            if ui
                                .add(egui::SelectableLabel::new(
                                    self.export_bools[0] == true,
                                    "All Titles",
                                ))
                                .clicked()
                            {
                                self.export_bools[0] = true;
                            }
                        });
                        ui.checkbox(&mut self.export_bools[1], "Include images");
                        ui.checkbox(&mut self.export_bools[2], "Include sources");
                        ui.checkbox(&mut self.export_bools[3], "Include dates");
                        ui.horizontal(|ui| {
                            if ui.button("Ok").clicked() {
                                if let Some(dir) = &self.export_directory {
                                    if !self.export_bools[0] {
                                        save_to_filename(
                                            dir.to_path_buf(),
                                            self.titles[self.current_title_index].name.clone(),
                                            self.export_title(
                                                self.titles[self.current_title_index].clone(),
                                            ),
                                        );
                                    } else {
                                        for title in self.titles.clone() {
                                            save_to_filename(
                                                dir.to_path_buf(),
                                                title.name.clone(),
                                                self.export_title(title),
                                            );
                                        }
                                    }
                                    self.popup_active = PopupActive::Empty;
                                }
                            }

                            if ui.button("✖ Close").clicked() {
                                self.popup_active = PopupActive::Empty;
                            }
                        });
                    });
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }

    fn export_title(&self, export_title: Title) -> String {
        let mut string_vec: Vec<String> = Vec::new();
        //Adding the title data
        string_vec.push(export_title.name);
        if self.export_bools[1] && export_title.image.path != "" {
            string_vec.push("Title image: ".to_owned() + &export_title.image.path);
            if export_title.image.description != "" {
                string_vec
                    .push("Title image description: ".to_owned() + &export_title.image.description);
            }
        }
        for point_id in export_title.point_ids {
            //Adding source
            if self.export_bools[2] {
                if self.points[&point_id].source != "" {
                    string_vec.push("Source: ".to_owned() + &self.points[&point_id].source.clone());
                }
            }
            //Adding images
            if self.export_bools[1] {
                if self.points[&point_id].images.len() > 0 {
                    for image in self.points[&point_id].images.clone() {
                        string_vec.push("Image: ".to_owned() + &image.path);
                        if image.description != "" {
                            string_vec.push("Image description: ".to_owned() + &image.description);
                        }
                    }
                }
            }
            //Adding dates
            if self.export_bools[3] {
                if let Some(date) = self.points[&point_id].date {
                    string_vec.push(date.to_string());
                }
            }
            //Adding content
            let mut local_content = self.points[&point_id].content.clone();
            local_content = local_content.replace("[!l]", "");
            local_content = local_content.replace("[!i]", "");
            local_content = local_content.replace("[!u]", "");
            local_content = local_content.replace("[!h]", "");
            local_content = local_content.replace("[!H]", "");
            string_vec.push(local_content);
        }
        let result = string_vec.join("\n");
        return result;
    }
}
