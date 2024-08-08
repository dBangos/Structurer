use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn point_datetime_popup(&mut self, ctx: &egui::Context) {
        if self.show_point_datetime_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Add Date and Time")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .min_size([300.0, 300.0])
                .max_size([300.0, 600.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(egui_extras::DatePickerButton::new(&mut self.point_date));
                        });
                        ui.horizontal(|ui| {
                            if ui.button("✅ Ok").clicked() {
                                self.show_point_datetime_popup = false;
                                self.current_points[self.point_requesting_action_index].date =
                                    Some(self.point_date);
                            }

                            if ui.button("✖ Close").clicked() {
                                self.show_point_datetime_popup = false;
                            }
                        });
                    });
                });
            self.show_point_datetime_popup &= show_popup;
        }
    }
}
