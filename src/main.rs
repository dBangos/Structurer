use crate::egui::{popup_below_widget, ComboBox, Id};
use crate::save_load::{
    add_point, add_title, change_title_name, delete_point, delete_title, load_from_filename,
    load_from_library, load_points_from_title_id, point_is_shared_with, save_to_filename,
    share_unshare_point,
};
use eframe::egui::{self};
use save_load::{link_unlink_title, title_is_linked_with};
use std::path::PathBuf;
mod save_load;

struct Structurer {
    project_directory: PathBuf,
    titles: Vec<String>,
    title_ids: Vec<String>,
    points_of_title: Vec<Vec<String>>,
    current_points: Vec<(String, String)>, //Current_point(point_id,point_content)
    current_title: String,
    current_title_id: String,
    age: i32,

    show_confirm_delete_popup: bool,
    point_requesting_deletion: String,

    show_share_point_popup: bool,
    point_requesting_sharing: String,
    titles_receiving_shared_point: Vec<bool>, //(title_id,title,is_shared_or_not)

    show_title_delete_popup: bool,
    show_link_title_popup: bool,
    titles_linked_to_current: Vec<bool>,
}

impl Default for Structurer {
    fn default() -> Self {
        Self {
            project_directory: Default::default(),
            titles: Vec::new(),
            title_ids: Vec::new(),
            points_of_title: Vec::new(),
            current_points: Vec::new(), //Current_point(point_id,point_content)
            current_title: String::new(),
            current_title_id: String::new(),
            age: 40,
            show_confirm_delete_popup: false,
            point_requesting_deletion: String::new(),
            show_share_point_popup: false,
            point_requesting_sharing: String::new(),
            titles_receiving_shared_point: Vec::new(),
            show_title_delete_popup: false,
            show_link_title_popup: false,
            titles_linked_to_current: Vec::new(),
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1820.0, 1000.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Structurer",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<Structurer>::default()
        }),
    )
}

impl eframe::App for Structurer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //Button Line
            ui.horizontal(|ui| {
                if ui.button("Set Project Directory").clicked() {
                    if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                        self.project_directory = dir_path;
                    }
                    (self.title_ids, self.titles, self.points_of_title) =
                        load_from_library(self.project_directory.clone());
                }
                if ui.button("Save").clicked() {
                    change_title_name(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                        self.current_title.clone(),
                    );
                    for (id, content) in self.current_points.clone().into_iter() {
                        save_to_filename(self.project_directory.clone(), id, content);
                    }
                }
                if ui.button("Add Point").clicked() {
                    self.current_points.push(add_point(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                    ));
                    (self.title_ids, self.titles, self.points_of_title) =
                        load_from_library(self.project_directory.clone());
                }
                if ui.button("Add Title").clicked() {
                    add_title(self.project_directory.clone());
                }
                if ui.button("Delete Title").clicked() {
                    self.show_title_delete_popup = true;
                }
                if ui.button("Save Page As:").clicked() {
                    self.age += 1;
                }
                if ui.button("Link Title").clicked() {
                    self.titles_linked_to_current = title_is_linked_with(
                        self.project_directory.clone(),
                        self.current_title_id.clone(),
                    );
                    self.show_link_title_popup = true;
                }
            });

            //Main layout, contains titles layout and points layout
            ui.horizontal(|ui| {
                //Titles layout ==========================================================
                ui.vertical(|ui| {
                    ui.label("All Titles");
                    ui.vertical(|ui| {
                        //Making sure tha data is clean
                        let temp_file_path_for_check: PathBuf =
                            [self.project_directory.clone(), PathBuf::from("Library.txt")]
                                .iter()
                                .collect();
                        if temp_file_path_for_check.exists() {
                            (self.title_ids, self.titles, self.points_of_title) =
                                load_from_library(self.project_directory.clone());
                        }
                        //Binding each title button to loading the corresponding points
                        for (title_id, title, t_points) in self
                            .title_ids
                            .clone()
                            .into_iter()
                            .zip(self.titles.clone().into_iter())
                            .zip(self.points_of_title.clone().into_iter())
                            .map(|((x, y), z)| (x, y, z))
                        {
                            if ui.button(title.clone()).clicked() {
                                //Saving the title of the curent page before switching
                                //First checking if the file exists
                                let temp_file_path_for_check: PathBuf = [
                                    self.project_directory.clone(),
                                    PathBuf::from(self.current_title_id.clone() + ".txt"),
                                ]
                                .iter()
                                .collect();
                                if temp_file_path_for_check.exists() {
                                    change_title_name(
                                        self.project_directory.clone(),
                                        self.current_title_id.clone(),
                                        self.current_title.clone(),
                                    );
                                }
                                //Setting the title field
                                self.current_title = title.clone();
                                self.current_title_id = title_id.clone();
                                //Updating the links for the new title_id
                                self.titles_linked_to_current = title_is_linked_with(
                                    self.project_directory.clone(),
                                    self.current_title_id.clone(),
                                );
                                //Save old points => Remove old points => Add new points
                                for (id, content) in self.current_points.clone().into_iter() {
                                    save_to_filename(self.project_directory.clone(), id, content);
                                }
                                self.current_points = Vec::new();
                                for new_point in t_points.into_iter() {
                                    self.current_points.push((
                                        new_point.to_string(),
                                        load_from_filename(
                                            new_point.to_string(),
                                            self.project_directory.clone(),
                                        ),
                                    ));
                                }
                            }
                        }
                    });
                    ui.label("Linked With:");
                    ui.vertical(|ui| {
                        //Binding each title button to loading the corresponding points
                        for (title_id, title, t_points, is_linked) in self
                            .title_ids
                            .clone()
                            .into_iter()
                            .zip(self.titles.clone().into_iter())
                            .zip(self.points_of_title.clone().into_iter())
                            .zip(self.titles_linked_to_current.clone().into_iter())
                            .map(|(((x, y), z), a)| (x, y, z, a))
                        {
                            if is_linked {
                                if ui.button(title.clone()).clicked() {
                                    //Saving the title of the curent page before switching
                                    //First checking if the file exists
                                    let temp_file_path_for_check: PathBuf = [
                                        self.project_directory.clone(),
                                        PathBuf::from(self.current_title_id.clone() + ".txt"),
                                    ]
                                    .iter()
                                    .collect();
                                    if temp_file_path_for_check.exists() {
                                        change_title_name(
                                            self.project_directory.clone(),
                                            self.current_title_id.clone(),
                                            self.current_title.clone(),
                                        );
                                    }
                                    //Setting the title field
                                    self.current_title = title.clone();
                                    self.current_title_id = title_id.clone();
                                    //Updating the links for the new title_id
                                    self.titles_linked_to_current = title_is_linked_with(
                                        self.project_directory.clone(),
                                        self.current_title_id.clone(),
                                    );
                                    //Save old points => Remove old points => Add new points
                                    for (id, content) in self.current_points.clone().into_iter() {
                                        save_to_filename(
                                            self.project_directory.clone(),
                                            id,
                                            content,
                                        );
                                    }
                                    self.current_points = Vec::new();
                                    for new_point in t_points.into_iter() {
                                        self.current_points.push((
                                            new_point.to_string(),
                                            load_from_filename(
                                                new_point.to_string(),
                                                self.project_directory.clone(),
                                            ),
                                        ));
                                    }
                                }
                            }
                        }
                    });
                });

                //All points layout==========================================
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let name_label = ui.label("Your name: ");
                        ui.text_edit_singleline(&mut self.current_title)
                            .labelled_by(name_label.id);
                    });
                    for point in self.current_points.iter_mut() {
                        // Container for elements of each point
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                if ui.button("Delete").clicked() {
                                    self.point_requesting_deletion = point.0.clone();
                                    self.show_confirm_delete_popup = true;
                                }
                                if ui.button("Add to:").clicked() {
                                    self.titles_receiving_shared_point = point_is_shared_with(
                                        self.project_directory.clone(),
                                        point.0.clone(),
                                    );
                                    self.point_requesting_sharing = point.0.clone();
                                    self.show_share_point_popup = true;
                                }
                            });
                            ui.text_edit_multiline(&mut point.1);
                        });
                    }
                });
            });

            // UI element examples that might be usefult later

            let response = ui.button("Open");
            let popup_id = Id::new("popup_id");

            if response.clicked() {
                ui.memory_mut(|mem| mem.toggle_popup(popup_id));
            }

            popup_below_widget(ui, popup_id, &response, |ui| {
                ui.set_min_width(300.0);
                ui.label("This popup will be open even if you click the checkbox");
            });

            ComboBox::from_label("ComboBox")
                .selected_text(format!("{}", self.age))
                .show_ui(ui, |ui| {
                    for num in 0..10 {
                        ui.selectable_value(&mut self.age, num, format!("{num}"));
                    }
                });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
        });
        if self.show_confirm_delete_popup {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Confirm Deletion")
                    .with_inner_size([300.0, 100.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Are you sure you want to permanently delete this point?");
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                delete_point(
                                    self.project_directory.clone(),
                                    self.point_requesting_deletion.clone(),
                                );
                                (self.title_ids, self.titles, self.points_of_title) =
                                    load_from_library(self.project_directory.clone());
                                self.current_points = load_points_from_title_id(
                                    self.project_directory.clone(),
                                    self.current_title_id.clone(),
                                );
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                ctx.request_repaint();
                            }

                            if ui.button("No").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_confirm_delete_popup = false;
                    }
                },
            );
        }
        if self.show_share_point_popup || self.show_link_title_popup {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Confirm Deletion")
                    .with_inner_size([200.0, 300.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if self.show_share_point_popup {
                            ui.label("Share point:");
                            ui.vertical(|ui| {
                                for (is_shared, title) in self
                                    .titles_receiving_shared_point
                                    .iter_mut()
                                    .zip(self.titles.clone())
                                {
                                    ui.checkbox(is_shared, title.clone());
                                }
                            });
                            ui.horizontal(|ui| {
                                if ui.button("Ok").clicked() {
                                    share_unshare_point(
                                        self.project_directory.clone(),
                                        self.point_requesting_sharing.clone(),
                                        self.titles_receiving_shared_point.clone(),
                                        self.title_ids.clone(),
                                    );
                                    //If the point is not shared to any titles, delete it
                                    if self
                                        .titles_receiving_shared_point
                                        .iter()
                                        .all(|c| *c == false)
                                    {
                                        delete_point(
                                            self.project_directory.clone(),
                                            self.point_requesting_sharing.clone(),
                                        );
                                    }
                                    (self.title_ids, self.titles, self.points_of_title) =
                                        load_from_library(self.project_directory.clone());
                                    self.current_points = load_points_from_title_id(
                                        self.project_directory.clone(),
                                        self.current_title_id.clone(),
                                    );
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    ctx.request_repaint();
                                }

                                if ui.button("Cancel").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            });
                        } else if self.show_link_title_popup {
                            ui.label("Link Title:");
                            ui.vertical(|ui| {
                                for (is_linked, title) in self
                                    .titles_linked_to_current
                                    .iter_mut()
                                    .zip(self.titles.clone())
                                {
                                    ui.checkbox(is_linked, title.clone());
                                }
                            });
                            ui.horizontal(|ui| {
                                if ui.button("Ok").clicked() {
                                    link_unlink_title(
                                        self.project_directory.clone(),
                                        self.current_title_id.clone(),
                                        self.titles_linked_to_current.clone(),
                                        self.title_ids.clone(),
                                    );
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                    ctx.request_repaint();
                                }

                                if ui.button("Cancel").clicked() {
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                            });
                        }
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_link_title_popup = false;
                    }
                },
            );
        }
        if self.show_title_delete_popup {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Confirm Deletion")
                    .with_inner_size([300.0, 100.0]),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Are you sure you want to permanently delete this title?");
                        ui.horizontal(|ui| {
                            if ui.button("Yes").clicked() {
                                delete_title(
                                    self.project_directory.clone(),
                                    self.current_title_id.clone(),
                                );
                                //Reseting the state and showing the first title
                                (self.title_ids, self.titles, self.points_of_title) =
                                    load_from_library(self.project_directory.clone());
                                (self.current_title_id, self.current_title) =
                                    (self.title_ids[0].clone(), self.titles[0].clone());
                                self.current_points = Vec::new();
                                for new_point in self.points_of_title[0].clone().into_iter() {
                                    self.current_points.push((
                                        new_point.to_string(),
                                        load_from_filename(
                                            new_point.to_string(),
                                            self.project_directory.clone(),
                                        ),
                                    ));
                                }
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                ctx.request_repaint();
                            }

                            if ui.button("No").clicked() {
                                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                            }
                        });
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_title_delete_popup = false;
                    }
                },
            );
        }
    }
}
