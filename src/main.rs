use chrono::{NaiveDate, NaiveTime};
use core::ops::Range;
use eframe::egui::{self};
use egui::{FontFamily, FontId, TextStyle};
use egui::{Pos2, Vec2};
use std::collections::HashMap;
use std::path::PathBuf;
use std::usize;
mod config;
mod gui_elements;
mod node_controls;
mod node_physics;
mod node_view;
mod popups {
    pub mod add_tags_popup;
    pub mod confirm_deletion_popup;
    pub mod node_view_popup;
    pub mod point_datetime_popup;
    pub mod point_image_popup;
    pub mod point_source_popup;
    pub mod share_point_link_title_popup;
    pub mod tags_popup;
    pub mod title_delete_popup;
    pub mod title_edit_popup;
    pub mod title_image_popup;
}
mod save_load {
    pub mod general;
    pub mod image;
    pub mod link;
    pub mod point;
    pub mod share;
    pub mod source;
    pub mod tag;
    pub mod title;
}

#[derive(Clone)]
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
    date: Option<NaiveDate>,
    time: Option<NaiveTime>,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            id: String::new(),
            content: String::new(),
            source: String::new(),
            images: Vec::new(),
            date: None,
            time: None,
        }
    }
}

#[derive(Clone)]
struct Title {
    name: String,
    id: String,
    point_ids: Vec<String>,
    links: Vec<bool>, //A vector of bools each correspondig to a title, if true it's linked
    node_screen_position: Pos2,
    node_physics_position: Vec2,
    node_currnetly_clicked: bool,
    image: ImageStruct,
    tags: Vec<String>,
}

impl Default for Title {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            point_ids: Vec::new(),
            links: Vec::new(),
            node_screen_position: Pos2::new(1.0, 1.0),
            node_physics_position: Vec2::new(0.0, 0.0),
            node_currnetly_clicked: false,
            image: ImageStruct::default(),
            tags: Vec::new(),
        }
    }
}
enum StateType {
    Empty,
    Title,
    Search,
    Timeline,
}

struct Structurer {
    project_directory: PathBuf,
    titles: Vec<Title>,
    points: HashMap<String, Point>,
    current_title_index: usize,
    current_point_ids: Vec<String>,
    show_confirm_delete_popup: bool,
    point_requesting_action_id: String,
    show_share_point_popup: bool,
    titles_receiving_shared_point: Vec<bool>,
    show_title_delete_popup: bool,
    show_link_title_popup: bool,
    show_source_popup: bool,
    show_title_image_popup: bool,
    show_point_image_popup: bool,
    show_title_edit_popup: bool,
    show_add_tags_popup: bool,
    point_image_requesting_popup: usize,
    drag_distance: Vec2,
    linked_pairs: Vec<(usize, usize)>,
    initialized: bool,
    view_scale: f32,
    stop_clicked_nodes: bool,
    all_tags: Vec<String>,
    current_title_tag_bools: Vec<bool>,
    possible_new_tag: String,
    node_view_start_stop_physics: bool,
    center_current_node: bool,
    show_node_view_popup: bool,
    show_tags_popup: bool,
    tags_actively_filtering: Vec<bool>,
    tags_in_filter: Vec<String>,
    show_point_datetime_popup: bool,
    point_popup_fields: (i32, u32, u32, u32, u32, u32),
    searching_string: String,
    current_state: StateType,
    next_page_point_ids: Vec<String>,
    point_id_being_edited: Option<String>,
    text_edit_cursor_range: Option<Range<usize>>,
}

impl Default for Structurer {
    fn default() -> Self {
        Self {
            current_state: StateType::Empty,
            project_directory: Default::default(),
            titles: Vec::new(),
            current_title_index: 0,
            current_point_ids: Vec::new(),
            points: HashMap::new(),
            point_requesting_action_id: String::new(),
            titles_receiving_shared_point: Vec::new(),
            point_image_requesting_popup: 0,
            linked_pairs: Vec::new(),
            initialized: false,
            all_tags: Vec::new(),
            current_title_tag_bools: Vec::new(),
            possible_new_tag: String::new(),
            tags_actively_filtering: Vec::new(),
            tags_in_filter: Vec::new(),
            point_popup_fields: (2024, 1, 1, 0, 0, 0),
            searching_string: String::new(),
            next_page_point_ids: Vec::new(),
            //Node view
            drag_distance: Vec2 { x: 0.0, y: 0.0 },
            stop_clicked_nodes: false,
            center_current_node: true,
            node_view_start_stop_physics: true,
            view_scale: 0.85,
            //Popup bools
            show_confirm_delete_popup: false,
            show_title_delete_popup: false,
            show_point_datetime_popup: false,
            show_node_view_popup: false,
            show_link_title_popup: false,
            show_source_popup: false,
            show_title_image_popup: false,
            show_point_image_popup: false,
            show_title_edit_popup: false,
            show_add_tags_popup: false,
            show_share_point_popup: false,
            show_tags_popup: false,
            point_id_being_edited: None,
            text_edit_cursor_range: None,
        }
    }
}

#[inline]
fn left_panel_labels() -> TextStyle {
    TextStyle::Name("LeftPanelLabels".into())
}
#[inline]
fn title_style() -> TextStyle {
    TextStyle::Name("TitleStyle".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(20.0, Proportional)),
        (left_panel_labels(), FontId::new(20.0, Proportional)),
        (TextStyle::Button, FontId::new(17.0, Proportional)),
        (title_style(), FontId::new(50.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../assets/icon.png")).unwrap();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1820.0, 1000.0])
            .with_icon(icon),
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
        configure_text_styles(ctx);
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
                .resizable(true)
                .default_width(150.0)
                .width_range(80.0..=400.0)
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add_space(15.0);
                        self.title_buttons(ui);
                        ui.add_space(15.0);
                        self.linked_titles_buttons(ui);
                    });
                });
            egui::SidePanel::right("right_panel")
                .resizable(true)
                .default_width(400.0)
                .width_range(80.0..=600.0)
                .show_inside(ui, |ui| {
                    self.node_controls(ui);

                    if !self.show_node_view_popup {
                        self.node_view(ui);
                        ctx.request_repaint();
                    }
                });
            egui::CentralPanel::default().show_inside(ui, |ui| {
                match self.current_state {
                    StateType::Empty => (),
                    StateType::Title => {
                        ui.vertical_centered(|ui| {
                            self.title_layout(ui);
                            ui.separator();

                            self.points_layout(ui);
                        });
                    }
                    StateType::Search | StateType::Timeline => {
                        //If searching show the results instead
                        self.points_layout(ui);
                    }
                }
            });
        });
        //Having all these ifs is ugly,but:
        //// They are different bools so I can &= easily with the bools for the x close button
        //// This might still be more elegant than using sth like a u8 and having to track which is which
        ////// Or using strings and doing more expensive comparisons:w
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
        if self.show_title_edit_popup {
            self.title_edit_popup(ctx);
        }
        if self.show_add_tags_popup {
            self.add_tags_popup(ctx);
        }
        if self.show_node_view_popup {
            self.node_view_popup(ctx);
        }
        if self.show_tags_popup {
            self.tags_popup(ctx);
        }
        if self.show_point_datetime_popup {
            self.point_datetime_popup(ctx);
        }
    }
}
