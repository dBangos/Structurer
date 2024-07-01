use crate::egui::{popup_below_widget, ComboBox, Id};
use eframe::egui::{self};
use std::path::PathBuf;
mod gui_elements;
mod popup_windows;
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
            self.main_button_line(ui);
            //Main layout, contains titles layout and points layout
            ui.horizontal(|ui| {
                //Titles layout ==========================================================
                ui.vertical(|ui| {
                    self.title_buttons(ui);
                    self.linked_titles_buttons(ui);
                });

                //All points layout==========================================
                self.points_layout(ui);
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
            self.confirm_deletion_popup(ctx);
        }
        if self.show_share_point_popup || self.show_link_title_popup {
            self.show_share_point_or_link_title_popup(ctx);
        }
        if self.show_title_delete_popup {
            self.title_delete_popup(ctx);
        }
    }
}
