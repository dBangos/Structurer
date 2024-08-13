use crate::save_load::title::save_title;
use crate::{left_panel_labels, PopupActive, Structurer};
use eframe::egui::{self, RichText};
impl Structurer {
    pub fn add_tags_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::AddTags {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Tags")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .min_size([600.0, 300.0])
                .max_size([600.0, 300.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_width(300.0);
                                ui.label(
                                    RichText::new("Add an existing tag")
                                        .text_style(left_panel_labels()),
                                );
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT)
                                            .with_main_wrap(true),
                                        |ui| {
                                            assert_eq!(
                                                self.all_tags.len(),
                                                self.current_title_tag_bools.len()
                                            );
                                            for (tag_bool, tag) in self
                                                .current_title_tag_bools
                                                .iter_mut()
                                                .zip(self.all_tags.clone())
                                            {
                                                ui.checkbox(tag_bool, tag);
                                            }
                                        },
                                    );
                                });
                            });
                            ui.separator();
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Create a new tag")
                                        .text_style(left_panel_labels()),
                                );

                                ui.text_edit_singleline(&mut self.possible_new_tag);
                                if ui.button("Create").clicked() {
                                    if self.possible_new_tag != String::new() {
                                        self.all_tags.push(self.possible_new_tag.clone());
                                        self.tags_actively_filtering.push(false);
                                        self.current_title_tag_bools.push(true);
                                        self.possible_new_tag = String::new();
                                    }
                                }
                            });
                        });
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(250.0);
                            if ui.button("✅ Ok").clicked() {
                                self.possible_new_tag = String::new();
                                self.titles[self.current_title_index].tags = Vec::new();
                                for (tag, tag_bool) in self
                                    .all_tags
                                    .clone()
                                    .into_iter()
                                    .zip(self.current_title_tag_bools.clone())
                                {
                                    if tag_bool {
                                        self.titles[self.current_title_index].tags.push(tag);
                                    }
                                }
                                save_title(
                                    self.project_directory.clone(),
                                    self.titles[self.current_title_index].clone(),
                                );
                                self.popup_active = PopupActive::Empty;
                            }
                            if ui.button("✖ Cancel").clicked() {
                                self.possible_new_tag = String::new();
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
