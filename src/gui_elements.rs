use crate::markup::{markup_construct_job, markup_parse_string};
use crate::save_load::image::add_image_to_point;
use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::{add_point, save_point};
use crate::save_load::share::point_is_shared_with;
use crate::save_load::title::{add_title, save_title};
use crate::{left_panel_labels, title_style, Structurer};
use crate::{ImageStruct, Point, Title};
use crate::{PopupActive, StateType};
use chrono::{Datelike, Timelike};
use core::f32;
use eframe::egui::{self, Button, RichText, TextWrapMode};
use egui::{Id, Key, TextEdit, Vec2};
use egui_dnd::{dnd, DragDropItem};
use rfd::FileDialog;

impl DragDropItem for &mut Title {
    fn id(&self) -> Id {
        Id::new(&self.id)
    }
}

impl DragDropItem for &mut Point {
    fn id(&self) -> Id {
        Id::new(&self.id)
    }
}
impl Structurer {
    //Button line that contains most basic functions
    pub fn main_button_line(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            if ui.button("🗀 Set Project Directory").clicked() {
                if let Some(dir_path) = rfd::FileDialog::new().pick_folder() {
                    //Resetting state in case old values don't get overwritten, in the absence of a
                    //previous library
                    self.titles = Vec::new();
                    self.current_title_index = 0;
                    self.view_scale = 0.85;
                    self.project_directory = dir_path;
                    self.current_state = StateType::Empty;
                    self.current_point_ids = Vec::new();
                    let _ = self.save_to_config(ctx);
                    self.create_library_files();
                    self.load_from_library();
                    self.get_all_points();
                    ui.ctx().forget_all_images();
                }
            }
            if ui.button("💾 Save").on_hover_text_at_pointer("Save data to file").clicked() {
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
                    //Saving here so save button updates the point_text_size on the json file
                    let _ = self.save_to_config(ctx);
                }
            }
            if ui.button("⤴ Export").on_hover_text_at_pointer("Export content to file").clicked(){
                match self.current_state {
                    StateType::Title => self.popup_active = PopupActive::Export,
                    _ => (),
                }
            }
            ui.separator();
            if ui.button("➕ Add Title").on_hover_text_at_pointer("Add a new title to this project").clicked() {
                //Create new title files
                let new_title_id = add_title(self.project_directory.clone());
                //Add new title to state
                let mut temp_title: Title = Title::default();
                temp_title.id = new_title_id.clone();
                temp_title.name = "New title".to_string();
                //Add point to the new title
                let temp_point = add_point(self.project_directory.clone(), &temp_title.id);
                if let Some(p) = temp_point {
                    self.points.insert(p.id.clone(), p.clone());
                    //Add new point to state
                    temp_title.point_ids.push(p.id);
                }
                //Switch focus to the new title page
                self.titles.push(temp_title);
                let last_idx = self.titles.len() - 1;
                match self.current_state {
                    StateType::Title => {
                        self.change_title(last_idx);
                    }
                    _ => {
                        let last_idx = self.titles.len() - 1;
                        if self.center_current_node {
                            self.drag_distance = -1.0
                                * self.titles[self.titles.len() - 1].node_physics_position
                                * self.view_scale;
                        }
                        self.next_page_point_ids = self.titles[last_idx].point_ids.clone();
                        self.save_old_add_new_points();
                        self.current_title_index = last_idx;
                        self.titles[last_idx].links = title_is_linked_with(
                            self.project_directory.clone(),
                            self.titles[last_idx].id.clone(),
                        );
                    }
                }
                self.current_title_index = self.titles.len() - 1;
            }
            if ui.button("↔ Link Title").on_hover_text_at_pointer("Create a link between this title and another").clicked() {
                match self.current_state {
                    StateType::Title => {
                        self.titles[self.current_title_index].links = title_is_linked_with(
                            self.project_directory.clone(),
                            self.titles[self.current_title_index].id.clone(),
                        );
                        self.popup_active = PopupActive::LinkTitle;
                    }
                    _ => (),
                }
            }
            if ui.button("🗑 Delete Title").on_hover_text_at_pointer("Permanently delete the current title. Any data not shared with other titles will also be deleted").clicked() {
                match self.current_state {
                    StateType::Title => self.popup_active = PopupActive::ConfirmTitleDeletion,
                    _ => (),
                }
            }
            ui.separator();
            if ui.button("+ Add Point").on_hover_text_at_pointer("Add a new point to the current title").clicked() {
                match self.current_state {
                    StateType::Title => {
                        if let Some(p) = add_point(
                            self.project_directory.clone(),
                            &self.titles[self.current_title_index].id,
                        ) {
                            self.points.insert(p.id.clone(), p.clone());
                            self.current_point_ids.push(p.id.clone());
                            self.titles[self.current_title_index].point_ids.push(p.id);
                        }
                    }
                    _ => (),
                }
            }
            ui.separator();
            if ui.button("📑 Tags").on_hover_text_at_pointer("Open the tags menu").clicked() {
                self.popup_active = PopupActive::TagsPopup;
            }
            ui.separator();
            if ui.button("📅 Timeline").on_hover_text_at_pointer("See the points in chronological order").clicked() {
                for id in self.current_point_ids.clone() {
                    save_point(self.project_directory.clone(), self.points[&id].clone());
                }
                self.current_state = StateType::Timeline;
                //Update the points to make sure the points are up to date
                //Filter and sort the points by date and time
                self.current_point_ids = Vec::new();
                for (key, val) in self.points.iter() {
                    if let Some(_date) = val.date {
                        self.current_point_ids.push(key.to_string());
                    }
                }
                self.current_point_ids.sort_by(|a, b| {
                    self.points[a]
                        .date
                        .cmp(&self.points[b].date)
                        .then(self.points[a].time.cmp(&self.points[b].time))
                });
            }
            ui.separator();
            let search_field = ui.add(TextEdit::singleline(&mut self.searching_string));
            //User can erase the string to end the search
            if self.searching_string == "" {
                match self.current_state {
                    StateType::Search => self.current_state = StateType::Empty,
                    _ => (),
                }
            }
            if ui.button("🔎 Search").on_hover_text_at_pointer("Search through the text of all points").clicked()
                || (search_field.lost_focus() && ui.input(|x| x.key_pressed(Key::Enter))){
                if self.searching_string != "" {
                    self.current_state = StateType::Search;
                    for id in self.current_point_ids.clone() {
                        save_point(self.project_directory.clone(), self.points[&id].clone());
                    }
                    //Filter and sort the points by date and time
                    self.current_point_ids = Vec::new();
                    for (key, val) in self.points.iter() {
                        if val.content.contains(&self.searching_string) {
                            self.current_point_ids.push(key.to_string());
                        }
                    }
                }
            }
            ui.separator();
        });
        match self.current_state {
            StateType::Title => {
                if self.tags_actively_filtering.iter().any(|&x| x == true) {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Only showing titles with tags:");
                        self.tags_in_filter = Vec::new();
                        for (tag_bool, tag) in self
                            .tags_actively_filtering
                            .iter_mut()
                            .zip(self.all_tags.clone())
                        {
                            if *tag_bool {
                                ui.checkbox(tag_bool, tag.clone());
                                self.tags_in_filter.push(tag);
                            }
                        }
                        if ui.button("↺ Reset").clicked() {
                            self.tags_actively_filtering = vec![false; self.all_tags.len()];
                            self.tags_in_filter = Vec::new();
                        }
                    });
                    ui.add_space(2.0);
                }
            }
            StateType::Search => {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Searching for points containing {}",
                        self.searching_string
                    ));
                    if ui.button("↺ Reset").clicked() {
                        self.current_state = StateType::Empty;
                        self.searching_string = String::new();
                    }
                });
                ui.add_space(2.0);
            }
            _ => (),
        }
        if let Some(point_id) = &self.point_id_being_edited.clone() {
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Italic").clicked() {
                    if let Some(range) = &self.text_edit_cursor_range {
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.start, "[!i]");
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.end + 4, "[!i]");
                    }
                }
                if ui.button("Underline").clicked() {
                    if let Some(range) = &self.text_edit_cursor_range {
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.start, "[!u]");
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.end + 4, "[!u]");
                    }
                }
                if ui.button("Highlight").clicked() {
                    if let Some(range) = &self.text_edit_cursor_range {
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.start, "[!l]");
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.end + 4, "[!l]");
                    }
                }
                ui.separator();
                if ui.button("Heading 1").clicked() {
                    if let Some(range) = &self.text_edit_cursor_range {
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.start, "\n[!H] ");
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.end + 6, "[!H]");
                    }
                }
                if ui.button("Heading 2").clicked() {
                    if let Some(range) = &self.text_edit_cursor_range {
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.start, "\n[!h] ");
                        self.points
                            .get_mut(point_id)
                            .unwrap()
                            .content
                            .insert_str(range.end + 6, "[!h]");
                    }
                }
            });
            ui.add_space(2.0);
        }
    }

    //Contains the list of buttons leading to all the titles
    pub fn title_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("All Titles")
                .text_style(left_panel_labels())
                .strong(),
        );
        ui.separator();
        ui.vertical(|ui| {
            let tag_filter = self.tags_actively_filtering.iter().any(|&x| x == true);
            let mut index: usize = 0;
            let mut index_of_button_clicked: Option<usize> = None;
            //Drag and drop functionality
            let response =
                dnd(ui, "dnd").show(self.titles.iter_mut(), |ui, title, handle, _state| {
                    //If the filter is active and the title has the tags
                    if (tag_filter
                        && self
                            .tags_in_filter
                            .iter()
                            .all(|item| title.tags.contains(item)))
                        || !tag_filter
                    {
                        handle.ui(ui, |ui| {
                            if ui
                                .add(
                                    Button::new(title.name.clone())
                                        .wrap_mode(TextWrapMode::Truncate),
                                )
                                .clicked()
                            {
                                match self.current_state {
                                    StateType::Title => (),
                                    _ => {
                                        self.current_state = StateType::Title;
                                        self.current_title_index = index;
                                    }
                                }
                                index_of_button_clicked = Some(index);
                            }
                        });
                    }
                    index += 1;
                });
            if let Some(update) = response.final_update() {
                self.change_title_position(update.from, update.to);
            }
            if let Some(idx) = index_of_button_clicked {
                self.change_title(idx);
            }
        });
    }

    //Contians the buttons leading to the currently displayed title's links
    pub fn linked_titles_buttons(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Linked Titles")
                .text_style(left_panel_labels())
                .strong(),
        );
        ui.separator();
        ui.vertical(|ui| {
            //Binding each title button to loading the corresponding points
            match self.current_state {
                StateType::Title => {
                    if self.titles.len() > 0 {
                        for (index, is_linked) in self.titles[self.current_title_index]
                            .links
                            .clone()
                            .into_iter()
                            .enumerate()
                        {
                            if is_linked && index < self.titles.len() {
                                //Binding each title button to loading the corresponding points
                                if ui
                                    .add(
                                        Button::new(self.titles[index].name.clone())
                                            .wrap_mode(TextWrapMode::Truncate),
                                    )
                                    .clicked()
                                {
                                    self.change_title(index);
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        });
    }

    //Contains the title image and fields
    pub fn title_layout(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            //If there is an image show it, else show the placeholder
            if self.titles[self.current_title_index].image.path.len() > 1 {
                let file_path = self.titles[self.current_title_index].image.path.clone();
                let image = egui::Image::new(format!("file://{file_path}"))
                    .fit_to_original_size(2.0)
                    .max_height(200.0)
                    .max_width(500.0)
                    .maintain_aspect_ratio(true)
                    .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.popup_active = PopupActive::TitleImage;
                }
            } else {
                let image =
                    egui::Image::new(egui::include_image!("../assets/plus-square-icon.png"))
                        .fit_to_exact_size([220.0, 220.0].into())
                        .sense(egui::Sense::click());
                if ui.add(image).clicked() {
                    self.popup_active = PopupActive::TitleImage;
                }
            }
            ui.add_space(10.0);
            ui.vertical(|ui| {
                if ui
                    .label(
                        RichText::new(self.titles[self.current_title_index].name.clone())
                            .text_style(title_style())
                            .strong(),
                    )
                    .on_hover_text_at_pointer("Click to modify")
                    .clicked()
                {
                    self.popup_active = PopupActive::TitleEdit;
                }
                ui.horizontal(|ui| {
                    //Add tag buttons
                    for tag in self.titles[self.current_title_index].tags.clone() {
                        //On click filter by tag
                        if ui.button(tag.clone()).clicked() {
                            //If the last checkbox got unchecked empty the string vector
                            if self.tags_actively_filtering.iter().all(|&x| x == false) {
                                self.tags_in_filter = Vec::new();
                            }
                            //If not already filtering with this tag, only then filter with it
                            if !self.tags_in_filter.contains(&tag) {
                                self.tags_in_filter.push(tag.clone());
                                assert_eq!(self.all_tags.len(), self.tags_actively_filtering.len());
                                if let Some(index) = self.all_tags.iter().position(|x| *x == tag) {
                                    self.tags_actively_filtering[index] = true;
                                }
                            }
                        }
                    }
                    let mut add_tag_label: String = "Add Tag".to_string();
                    if self.titles[self.current_title_index].tags.len() > 0 {
                        add_tag_label = "+".to_string();
                    }
                    if ui.button(add_tag_label).clicked() {
                        self.current_title_tag_bools = Vec::new();
                        for tag in self.all_tags.clone() {
                            if self.titles[self.current_title_index].tags.contains(&tag) {
                                self.current_title_tag_bools.push(true);
                            } else {
                                self.current_title_tag_bools.push(false);
                            }
                        }
                        self.popup_active = PopupActive::AddTags;
                    }
                });
            });
        });
    }

    //Contains all the points and their buttons
    pub fn points_layout(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            //Adding an element to force the scrollbar to be on the left edge
            ui.add_sized([ui.available_size().x, 0.01], egui::Separator::default());
            ui.vertical(|ui| {
                let response = dnd(ui, "dnd2").show(
                    self.current_point_ids.iter_mut(),
                    |ui, point_id, handle, _state| {
                        // Container for elements of each point
                        ui.add_space(5.0);
                        match self.current_state {
                            StateType::Timeline => {
                                if let Some(date) = self.points[point_id].date {
                                    if let Some(time) = self.points[point_id].time {
                                        ui.label(format!(
                                            "{} - {}",
                                            date.to_string(),
                                            time.to_string()
                                        ));
                                    } else {
                                        ui.label(format!("{}", date.to_string()));
                                    }
                                }
                            }
                            _ => (),
                        }
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                //Buttons
                                ui.horizontal(|ui| {
                                    handle.ui(ui, |ui| {
                                        ui.label("↕").on_hover_text_at_pointer(
                                            "Click and drag to change the point's position",
                                        );
                                    });
                                    ui.vertical(|ui| {
                                        ui.menu_button("⏷ More", |ui| {
                                            if ui.button("🖼 Add Images").clicked() {
                                                ui.close_menu();
                                                if let Some(files) = FileDialog::new()
                                                    .add_filter(
                                                        "image",
                                                        &["jpeg", "jpg", "png", "webp"],
                                                    )
                                                    .set_directory(self.project_directory.clone())
                                                    .pick_files()
                                                {
                                                    for file in files {
                                                        let mut new_image: ImageStruct =
                                                            ImageStruct::default();
                                                        new_image.path =
                                                            file.to_string_lossy().to_string();
                                                        self.points
                                                            .get_mut(point_id)
                                                            .unwrap()
                                                            .images
                                                            .push(new_image.clone());
                                                        add_image_to_point(
                                                            self.project_directory.clone(),
                                                            point_id.clone(),
                                                            new_image,
                                                        );
                                                    }
                                                }
                                            }
                                            if ui.button("📆 Add Date").clicked() {
                                                self.point_requesting_action_id =
                                                    point_id.to_string();
                                                if let Some(date) = self.points[point_id].date {
                                                    self.point_popup_fields.0 = date.year();
                                                    self.point_popup_fields.1 = date.month();
                                                    self.point_popup_fields.2 = date.day();
                                                }
                                                if let Some(time) = self.points[point_id].time {
                                                    self.point_popup_fields.3 = time.hour();
                                                    self.point_popup_fields.4 = time.minute();
                                                    self.point_popup_fields.5 = time.second();
                                                }
                                                self.popup_active = PopupActive::PointDateTime;
                                            }
                                            if ui.button("🔀 Share").clicked() {
                                                self.titles_receiving_shared_point =
                                                    point_is_shared_with(
                                                        self.project_directory.clone(),
                                                        point_id.clone(),
                                                    );
                                                self.point_requesting_action_id =
                                                    point_id.to_string();
                                                self.popup_active = PopupActive::SharePoint;
                                            }
                                            if ui.button("ℹ Source").clicked() {
                                                self.point_requesting_action_id =
                                                    point_id.to_string();
                                                self.popup_active = PopupActive::PointSource;
                                            }
                                            if ui.button("🗑 Delete").clicked() {
                                                self.point_requesting_action_id =
                                                    point_id.to_string();
                                                self.popup_active =
                                                    PopupActive::ConfirmPointDeletion;
                                            }
                                        });
                                        match self.point_id_being_edited.clone() {
                                            Some(p_id) if p_id == *point_id => {
                                                if ui.button("✅ Ok").clicked() {
                                                    self.point_id_being_edited = None;
                                                }
                                            }
                                            _ => {
                                                if ui.button("✏ Edit").clicked() {
                                                    self.point_id_being_edited =
                                                        Some(point_id.to_string());
                                                }
                                            }
                                        }
                                    });
                                });
                                ui.horizontal(|ui| {
                                    ui.add_space(30.0);
                                    if let Some(_date) = self.points[point_id].date {
                                        let image = egui::Image::new(egui::include_image!(
                                            "../assets/calendar-checkmark-icon.png"
                                        ))
                                        .fit_to_exact_size([20.0, 20.0].into())
                                        .sense(egui::Sense::click());
                                        if ui
                                            .add(image)
                                            .on_hover_text_at_pointer(
                                                "This point has a date, click to show",
                                            )
                                            .clicked()
                                        {
                                            self.popup_active = PopupActive::PointDateTime;
                                        }
                                    }
                                    if self.points[point_id].source != "" {
                                        let image = egui::Image::new(egui::include_image!(
                                            "../assets/info-circle-icon.png"
                                        ))
                                        .fit_to_exact_size([20.0, 20.0].into())
                                        .sense(egui::Sense::click());
                                        if ui
                                            .add(image)
                                            .on_hover_text_at_pointer(
                                                "This point has a source, click to show",
                                            )
                                            .clicked()
                                        {
                                            self.popup_active = PopupActive::PointSource;
                                        }
                                    }
                                });
                            });
                            ui.vertical(|ui| {
                                ui.style_mut().spacing.item_spacing = Vec2::new(1.0, 1.0);
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::LEFT)
                                        .with_main_wrap(true),
                                    |ui| {
                                        for (image_index, image) in self.points[point_id]
                                            .images
                                            .clone()
                                            .into_iter()
                                            .enumerate()
                                        {
                                            let file_path = image.path.clone();
                                            let curr_image =
                                                egui::Image::new(format!("file://{file_path}"))
                                                    .fit_to_original_size(2.0)
                                                    .max_height(70.0)
                                                    .sense(egui::Sense::click());

                                            if ui.add(curr_image).clicked() {
                                                self.point_requesting_action_id =
                                                    point_id.to_string();
                                                self.point_image_requesting_popup = image_index;
                                                self.popup_active = PopupActive::PointImage;
                                            }
                                        }
                                    },
                                );
                                //Avoiding borrowing self again
                                //Only show the editor for the clicked point
                                match self.point_id_being_edited.clone() {
                                    Some(p_id) if p_id == *point_id => {
                                        let output = egui::TextEdit::multiline(
                                            &mut self.points.get_mut(point_id).unwrap().content,
                                        )
                                        .desired_width(f32::INFINITY)
                                        .desired_rows(2)
                                        .show(ui);
                                        //Passing the option to unwrap later doesn't work so leave
                                        //it like this
                                        if let Some(text_cursor_range) = output.cursor_range {
                                            self.text_edit_cursor_range =
                                                Some(text_cursor_range.as_sorted_char_range());
                                        }
                                    }
                                    _ => {
                                        let response =
                                            ui.label(markup_construct_job(markup_parse_string(
                                                self.points[point_id].content.clone(),
                                            )));
                                        if response
                                            .on_hover_text_at_pointer("Click to modify")
                                            .clicked()
                                        {
                                            self.point_id_being_edited = Some(point_id.to_string());
                                        }
                                    }
                                }
                            });
                        });
                    },
                );
                if let Some(update) = response.final_update() {
                    self.change_point_position(update.from, update.to);
                }
            });
            ui.separator();
            if ui
                .button("+ Add Point")
                .on_hover_text_at_pointer("Add a new point to the current title")
                .clicked()
            {
                match self.current_state {
                    StateType::Title => {
                        if let Some(p) = add_point(
                            self.project_directory.clone(),
                            &self.titles[self.current_title_index].id,
                        ) {
                            self.points.insert(p.id.clone(), p.clone());
                            self.current_point_ids.push(p.id.clone());
                            self.titles[self.current_title_index].point_ids.push(p.id);
                        }
                    }
                    _ => (),
                }
            }
        });
    }
}
