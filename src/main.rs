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
    show_source_popup: bool,
    point_requesting_source: String,
    point_source: String,
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
            show_source_popup: false,
            point_requesting_source: String::new(),
            point_source: String::new(),
        }
    }
}

use egui::{FontFamily, FontId, TextStyle};

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(17.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
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
            configure_text_styles(&cc.egui_ctx);
            Box::<Structurer>::default()
        }),
    )
}

impl eframe::App for Structurer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //Main layout
            ui.vertical(|ui| {
                //Button Line
                self.main_button_line(ui);
                //Contains titles layout and points layout
                ui.horizontal(|ui| {
                    //Titles layout ==========================================================
                    ui.vertical(|ui| {
                        self.title_buttons(ui);
                        self.linked_titles_buttons(ui);
                    });

                    //All points layout==========================================
                    self.points_layout(ui);
                });
            });
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
        if self.show_source_popup {
            self.point_source_popup(ctx);
        }
    }
}
