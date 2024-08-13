use crate::save_load::link::{get_linked_pairs, link_unlink_title};
use crate::save_load::point::delete_point;
use crate::save_load::share::share_unshare_point;
use crate::{PopupActive, Structurer};
use eframe::egui::{self};
impl Structurer {
    pub fn show_share_point_or_link_title_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::SharePoint
            || self.popup_active == PopupActive::LinkTitle
        {
            let mut show_popup = true;
            egui::Window::new("")
                .resizable(false)
                .default_pos([700.0, 200.0])
                .min_size([300.0, 300.0])
                .max_size([300.0, 600.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    if self.popup_active == PopupActive::SharePoint {
                        ui.label("Share point:");
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                for (is_shared, checkbox_title) in self
                                    .titles_receiving_shared_point
                                    .iter_mut()
                                    .zip(self.titles.clone())
                                {
                                    ui.checkbox(is_shared, checkbox_title.name.clone());
                                }
                            });
                        });
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(70.0);
                            if ui.button("✅ Share").clicked() {
                                share_unshare_point(
                                    self.project_directory.clone(),
                                    self.point_requesting_action_id.clone(),
                                    self.titles_receiving_shared_point.clone(),
                                );
                                //Adding the point to shared in state, removing from unshared in
                                //state
                                for (title, is_shared) in self
                                    .titles
                                    .iter_mut()
                                    .zip(self.titles_receiving_shared_point.clone())
                                {
                                    if is_shared
                                        && !title
                                            .point_ids
                                            .contains(&self.point_requesting_action_id)
                                    {
                                        title
                                            .point_ids
                                            .push(self.point_requesting_action_id.clone());
                                    } else if !is_shared
                                        && title
                                            .point_ids
                                            .contains(&self.point_requesting_action_id)
                                    {
                                        title.point_ids.retain(|x| {
                                            *x != self.point_requesting_action_id.clone()
                                        });
                                    }
                                }
                                //If the point is not shared to any titles, delete it
                                if self
                                    .titles_receiving_shared_point
                                    .iter()
                                    .all(|c| *c == false)
                                {
                                    delete_point(
                                        self.project_directory.clone(),
                                        self.point_requesting_action_id.clone(),
                                    );
                                    //Removing the point from all titles in state
                                    for title in self.titles.iter_mut() {
                                        title
                                            .point_ids
                                            .retain(|x| *x != self.point_requesting_action_id)
                                    }
                                }
                                //Refresh the current title
                                self.change_title(self.current_title_index);
                                self.popup_active = PopupActive::Empty;
                            }

                            if ui.button("✖ Cancel").clicked() {
                                self.popup_active = PopupActive::Empty;
                            }
                        });
                    } else if self.popup_active == PopupActive::LinkTitle {
                        ui.label("Link Title:");
                        ui.vertical(|ui| {
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    //Need to create a local temp to obey the borrow checkers whims
                                    let mut local_name_list: Vec<String> = Vec::new();
                                    for title in self.titles.clone() {
                                        local_name_list.push(title.name);
                                    }
                                    for (is_linked, title_name) in self.titles
                                        [self.current_title_index]
                                        .links
                                        .iter_mut()
                                        .zip(local_name_list)
                                    {
                                        ui.checkbox(is_linked, title_name);
                                    }
                                });
                            });
                        });
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(75.0);
                            if ui.button("✅ Link").clicked() {
                                link_unlink_title(
                                    self.project_directory.clone(),
                                    self.current_title_index.clone(),
                                    self.titles.clone(),
                                );
                                self.popup_active = PopupActive::Empty;
                                self.linked_pairs = get_linked_pairs(
                                    self.project_directory.clone(),
                                    self.titles.clone(),
                                );
                            }

                            if ui.button("✖ Cancel").clicked() {
                                self.popup_active = PopupActive::Empty;
                            }
                        });
                    }
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }
}
