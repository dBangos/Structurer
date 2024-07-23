use eframe::egui::{self};
use std::path::PathBuf;
mod config;
mod gui_elements;
mod node_view;
mod popup_windows;
mod save_load {
    pub mod general;
    pub mod image;
    pub mod link;
    pub mod point;
    pub mod share;
    pub mod source;
    pub mod title;
}
use egui::{Pos2, Vec2};
use std::collections::HashMap;

#[derive(Clone)]
//Changed this to ImageStruct so as not to match egui::Image
struct ImageStruct {
    path: String,
    description: String,
}
impl Default for ImageStruct {
    fn default() -> Self {
        Self {
            path: String::new(),
            description: String::new(),
        }
    }
}

#[derive(Clone)]
struct Point {
    id: String,
    content: String,
    source: String,
    images: Vec<ImageStruct>,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            id: String::new(),
            content: String::new(),
            source: String::new(),
            images: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Title {
    name: String,
    id: String,
    point_ids: Vec<String>,
    links: Vec<bool>, //A vector of bools each correspondig to a title, if true it's linked
    node_position: Pos2,
    image: ImageStruct,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            point_ids: Vec::new(),
            links: Vec::new(),
            node_position: Pos2::new(1.0, 1.0),
            image: ImageStruct::default(),
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
    show_title_image_popup: bool,
    show_point_image_popup: bool,
    point_image_requesting_popup: (usize, usize), //Index of point in title, index of image in point
    drag_distance: Vec2,
    initialized: bool,
    view_scale: f32,
    point_text_size: f32,
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
            show_title_image_popup: false,
            show_point_image_popup: false,
            point_image_requesting_popup: (0, 0),
            drag_distance: Vec2 { x: 0.0, y: 0.0 },
            initialized: false,
            view_scale: 1.0,
            point_text_size: 20.0,
        }
    }
}

use egui::{FontFamily, FontId, TextStyle};
#[inline]
fn left_panel_labels() -> TextStyle {
    TextStyle::Name("LeftPanelLabels".into())
}
#[inline]
fn point_style() -> TextStyle {
    TextStyle::Name("PointStyle".into())
}

fn configure_text_styles(ctx: &egui::Context, point_text_size: f32) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (left_panel_labels(), FontId::new(20.0, Proportional)),
        (TextStyle::Button, FontId::new(17.0, Proportional)),
        (point_style(), FontId::new(point_text_size, Proportional)),
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
        configure_text_styles(ctx, self.point_text_size);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        self.main_button_line(ui);
                        ui.separator();
                        self.text_settings_line(ui);
                    });
                });
            egui::SidePanel::left("left_panel")
                .resizable(false)
                .default_width(150.0)
                .width_range(80.0..=400.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.add_space(15.0);
                            self.title_buttons(ui);
                            ui.add_space(15.0);
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
                //Don't render anything if no title is loaded
                if self.current_title.id.len() > 1 {
                    ui.vertical_centered(|ui| {
                        self.title_layout(ui);
                        ui.separator();

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            self.points_layout(ui);
                        });
                    });
                }
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
        if self.show_title_image_popup {
            self.title_image_popup(ctx);
        }
        if self.show_point_image_popup {
            self.point_image_popup(ctx);
        }
    }
}
