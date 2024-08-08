use crate::Structurer;
use eframe::egui::{self};
use egui::Vec2;
impl Structurer {
    pub fn timeline_popup(&mut self, ctx: &egui::Context) {
        if self.show_timeline_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Timeline")
                .resizable(true)
                .default_pos([900.0, 400.0])
                .min_size([500.0, 500.0])
                .max_height(700.0)
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for point in self.all_points.clone() {
                                ui.vertical(|ui| {
                                    if let Some(date) = point.date {
                                        if let Some(time) = point.time {
                                            ui.label(format!(
                                                "{} - {}",
                                                date.to_string(),
                                                time.to_string()
                                            ));
                                        } else {
                                            ui.label(format!("{}", date.to_string()));
                                        }
                                    }
                                    ui.style_mut().spacing.item_spacing = Vec2::new(1.0, 1.0);
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT)
                                            .with_main_wrap(true),
                                        |ui| {
                                            for image in point.images.clone() {
                                                let file_path = image.path.clone();
                                                let curr_image =
                                                    egui::Image::new(format!("file://{file_path}"))
                                                        .fit_to_original_size(2.0)
                                                        .max_height(70.0)
                                                        .sense(egui::Sense::click());

                                                ui.add(curr_image);
                                            }
                                        },
                                    );
                                    ui.label(point.content);
                                    ui.separator();
                                });
                            }
                        });
                        if ui.button("✖ Close").clicked() {
                            self.show_timeline_popup = false;
                        }
                    });
                });
            self.show_timeline_popup &= show_popup;
        }
    }
}
