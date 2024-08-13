use crate::save_load::link::get_linked_pairs;
use crate::save_load::title::delete_title;
use crate::Structurer;
use eframe::egui::{self};
impl Structurer {
    pub fn title_delete_popup(&mut self, ctx: &egui::Context) {
        egui::Window::new("Confirm Deletion")
            .resizable(false)
            .default_pos([900.0, 400.0])
            .show(ctx, |ui| {
                ui.label("Are you sure you want to permanently delete this title?");
                ui.horizontal(|ui| {
                    ui.add_space(85.0);
                    if ui.button("🗑 Delete").clicked() {
                        let delete_title_index = self.current_title_index;
                        self.change_title(0);
                        self.current_title_index = 0;
                        delete_title(
                            self.project_directory.clone(),
                            self.titles[delete_title_index].id.clone(),
                        );
                        //Removing the title from state
                        self.titles.remove(delete_title_index);
                        //Updating linked pairs
                        self.linked_pairs =
                            get_linked_pairs(self.project_directory.clone(), self.titles.clone());
                        self.show_title_delete_popup = false;
                    }
                    if ui.button("✖ Cancel").clicked() {
                        self.show_title_delete_popup = false;
                    }
                });
            });
    }
}
