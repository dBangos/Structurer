use eframe::egui::{self};
use std::path::PathBuf;
mod config;
mod gui_elements;
mod node_view;
mod popup_windows;
mod save_load;
use egui::emath::TSTransform;
use egui::{Pos2, Vec2};
use std::collections::HashMap;
#[derive(Clone)]
struct Point {
    id: String,
    content: String,
    source: String,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            id: String::new(),
            content: String::new(),
            source: String::new(),
        }
    }
}

#[derive(Clone)]
struct Title {
    name: String,
    id: String,
    point_ids: Vec<String>,
    links: Vec<bool>, //A vectir of bools each correspondig to a title, if true it's linked
    node_position: Pos2,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            point_ids: Vec::new(),
            links: Vec::new(),
            node_position: Pos2::new(0.0, 0.0),
        }
    }
}

struct Structurer {
    project_directory: PathBuf,
    titles: HashMap<String, Title>,
    title_order: Vec<String>,
    current_points: Vec<Point>, //Current_point(point_id,point_content)
    current_title: Title,

    show_confirm_delete_popup: bool,
    point_requesting_action_index: usize, //The index of the point in current_points
    show_share_point_popup: bool,
    titles_receiving_shared_point: Vec<bool>, //(title_id,title,is_shared_or_not)

    show_title_delete_popup: bool,
    show_link_title_popup: bool,
    show_source_popup: bool,
    drag_distance: Vec2,
    initialized: bool,
    view_scale: f32,
}

impl Default for Structurer {
    fn default() -> Self {
        Self {
            project_directory: Default::default(),
            titles: HashMap::new(),
            title_order: Vec::new(),
            current_points: Vec::new(), //Current_point(point_id,point_content)
            current_title: Title::default(),
            show_confirm_delete_popup: false,
            point_requesting_action_index: 0,
            show_share_point_popup: false,
            titles_receiving_shared_point: Vec::new(),
            show_title_delete_popup: false,
            show_link_title_popup: false,
            show_source_popup: false,
            drag_distance: Vec2 { x: 0.0, y: 0.0 },
            initialized: false,
            view_scale: 1.0,
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
            Ok(Box::<Structurer>::default())
        }),
    )
}

impl eframe::App for Structurer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.initialized {
            self.start_routine();
            self.initialized = true;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        self.main_button_line(ui);
                    });
                });
            egui::SidePanel::left("left_panel")
                .resizable(false)
                .default_width(150.0)
                .width_range(80.0..=400.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical(|ui| {
                            self.title_buttons(ui);
                            self.linked_titles_buttons(ui);
                        });
                    });
                });
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(400.0)
                .width_range(80.0..=600.0)
                .show_inside(ui, |ui| {
                    self.node_view(ui);
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.text_edit_singleline(&mut self.current_title.name);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.points_layout(ui);
                    });
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
