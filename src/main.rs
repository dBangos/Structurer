use crate::save_load::point::save_point;
use crate::save_load::title::save_title;
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
    pub mod export_popup;
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
    pub mod tag;
    pub mod title;
}
mod markup;

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

#[derive(PartialEq)]
enum PopupActive {
    Empty,
    ConfirmPointDeletion,
    ConfirmTitleDeletion,
    PointDateTime,
    LinkTitle,
    PointSource,
    TitleImage,
    PointImage,
    TitleEdit,
    AddTags,
    TagsPopup,
    SharePoint,
    Export,
}

struct Structurer {
    project_directory: PathBuf,
    titles: Vec<Title>,
    points: HashMap<String, Point>,
    current_title_index: usize,
    current_point_ids: Vec<String>,
    point_requesting_action_id: String,
    titles_receiving_shared_point: Vec<bool>,
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
    tags_actively_filtering: Vec<bool>,
    tags_in_filter: Vec<String>,
    point_popup_fields: (i32, u32, u32, u32, u32, u32),
    searching_string: String,
    current_state: StateType,
    next_page_point_ids: Vec<String>,
    point_id_being_edited: Option<String>,
    text_edit_cursor_range: Option<Range<usize>>,
    popup_active: PopupActive,
    export_directory: Option<PathBuf>,
    export_bools: [bool; 4],
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
            point_id_being_edited: None,
            text_edit_cursor_range: None,
            export_directory: None,
            export_bools: [false, false, false, false],
            //Node view
            drag_distance: Vec2 { x: 0.0, y: 0.0 },
            stop_clicked_nodes: false,
            center_current_node: true,
            node_view_start_stop_physics: true,
            view_scale: 0.85,
            show_node_view_popup: false,
            popup_active: PopupActive::Empty,
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

fn configure_style(ctx: &egui::Context) {
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
            self.start_routine(ctx);
            self.initialized = true;
        }
        configure_style(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel")
                .resizable(false)
                .min_height(32.0)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        self.main_button_line(ui, ctx);
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
        //This is a window instead of a popup
        //Should be able to be active while another popup is active
        if self.show_node_view_popup {
            self.node_view_popup(ctx);
        }
        match self.popup_active {
            PopupActive::Empty => (),
            PopupActive::ConfirmTitleDeletion => self.title_delete_popup(ctx),
            PopupActive::ConfirmPointDeletion => self.confirm_deletion_popup(ctx),
            PopupActive::LinkTitle => self.show_share_point_or_link_title_popup(ctx),
            PopupActive::SharePoint => self.show_share_point_or_link_title_popup(ctx),
            PopupActive::PointSource => self.point_source_popup(ctx),
            PopupActive::AddTags => self.add_tags_popup(ctx),
            PopupActive::TagsPopup => self.tags_popup(ctx),
            PopupActive::TitleImage => self.title_image_popup(ctx),
            PopupActive::PointImage => self.point_image_popup(ctx),
            PopupActive::PointDateTime => self.point_datetime_popup(ctx),
            PopupActive::TitleEdit => self.title_edit_popup(ctx),
            PopupActive::Export => self.export_popup(ctx),
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if let Some(()) = save_title(
            self.project_directory.clone(),
            self.titles[self.current_title_index].clone(),
        ) {
            for point_id in self.current_point_ids.clone() {
                save_point(
                    self.project_directory.clone(),
                    self.points[&point_id].clone(),
                );
            }
        }
    }
}
