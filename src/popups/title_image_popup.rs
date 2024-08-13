use crate::Structurer;
use eframe::egui::{self};
use rfd::FileDialog;
impl Structurer {
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
                            .add_filter("image", &["jpeg", "jpg", "png", "webp"])
                            .set_directory(self.project_directory.clone())
                            .pick_file();
                        self.titles[self.current_title_index].image.path =
                            file.unwrap_or_default().to_string_lossy().to_string();
                    }
                });
            });
    }
}
