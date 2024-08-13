use crate::{left_panel_labels, Structurer};
use eframe::egui::{self, RichText};
impl Structurer {
    pub fn tags_popup(&mut self, ctx: &egui::Context) {
        if self.show_tags_popup {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Tags")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .min_size([300.0, 300.0])
                .max_size([300.0, 600.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new("Select tags to filter:")
                                        .text_style(left_panel_labels()),
                                );
                                egui::ScrollArea::vertical().show(ui, |ui| {
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::LEFT)
                                            .with_main_wrap(true),
                                        |ui| {
                                            assert_eq!(
                                                self.all_tags.len(),
                                                self.tags_actively_filtering.len()
                                            );
                                            for (tag_bool, tag) in self
                                                .tags_actively_filtering
                                                .iter_mut()
                                                .zip(self.all_tags.clone())
                                            {
                                                ui.checkbox(tag_bool, tag);
                                            }
                                        },
                                    );
                                });
                            });
                        });
                        if ui.button("✖ Close").clicked() {
                            self.possible_new_tag = String::new();
                            self.show_tags_popup = false;
                        }
                    });
                });
            self.show_tags_popup &= show_popup;
        }
    }
}
