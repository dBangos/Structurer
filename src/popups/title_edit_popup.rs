use crate::{PopupActive, Structurer};
use eframe::egui::{self};
impl Structurer {
    pub fn title_edit_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::TitleEdit {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Please enter a new name")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.text_edit_singleline(&mut self.titles[self.current_title_index].name);
                        ui.horizontal(|ui| {
                            if ui.button("✅ Ok").clicked() {
                                //Making sure it can't be empty and impossible to click
                                if self.titles[self.current_title_index].name == "".to_string() {
                                    self.titles[self.current_title_index].name =
                                        "New title".to_string();
                                }
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
}
