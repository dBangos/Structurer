use crate::save_load::image::delete_image_from_point;
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
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
}
