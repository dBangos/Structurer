use core::f32;

use crate::{PopupActive, Structurer};
use eframe::egui::{self};
use rfd::FileDialog;
impl Structurer {
    pub fn title_image_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::TitleImage {
            let mut show_popup = true;
            egui::Window::new("")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        //If there is an image attached, replace the placeholder
                        if self.titles[self.current_title_index].image.path.len() > 1 {
                            let file_path =
                                self.titles[self.current_title_index].image.path.clone();
                            let image = egui::Image::new(format!("file://{file_path}"))
                                .fit_to_exact_size([600.0, 600.0].into())
                                .sense(egui::Sense::click());
                            ui.add(image);
                        } else {
                            let image = egui::Image::new(egui::include_image!(
                                "../../assets/plus-square-icon.png"
                            ))
                            .fit_to_exact_size([220.0, 220.0].into())
                            .sense(egui::Sense::click());
                            if ui.add(image).clicked() {
                                let file = FileDialog::new()
                                    .add_filter("image", &["jpeg", "jpg", "png", "webp"])
                                    .set_directory(self.project_directory.clone())
                                    .pick_file();
                                self.titles[self.current_title_index].image.path =
                                    file.unwrap_or_default().to_string_lossy().to_string();
                            }
                            if ui.button("🖼 Load Image").clicked() {
                                let file = FileDialog::new()
                                    .add_filter("image", &["jpeg", "jpg", "png", "webp"])
                                    .set_directory(self.project_directory.clone())
                                    .pick_file();
                                self.titles[self.current_title_index].image.path =
                                    file.unwrap_or_default().to_string_lossy().to_string();
                            }
                        }
                    });
                    ui.label("Description");
                    ui.vertical_centered_justified(|ui| {
                        ui.add(
                            egui::TextEdit::multiline(
                                &mut self.titles[self.current_title_index].image.description,
                            )
                            .desired_width(f32::INFINITY),
                        );
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 2.0 - 60.0);
                        if ui.button("✅ Ok").clicked() {
                            self.popup_active = PopupActive::Empty;
                        }
                        if ui.button("🔄 Reset").clicked() {
                            self.titles[self.current_title_index].image.description = String::new();
                            self.titles[self.current_title_index].image.path = String::new();
                        }
                    });
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }
}
