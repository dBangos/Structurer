use crate::save_load::link::title_is_linked_with;
use crate::save_load::point::save_point;
use crate::save_load::title::save_title;
use crate::{StateType, Structurer, Title};
use eframe::egui::{self, Pos2};
use egui::emath::RectTransform;
use egui::{Color32, FontId, Frame, Rect, Rounding, Sense, Shape, Stroke, Vec2};
impl Structurer {
    pub fn node_view(&mut self, ui: &mut egui::Ui) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                Vec2::new(ui.available_width(), ui.available_height()),
                Sense::click_and_drag(),
            );
            ui.set_clip_rect(response.rect);
            // Allow dragging the background
            if response.dragged() {
                self.drag_distance += response.drag_delta();
            }
            //Translate points to screen coordinates
            let to_screen = RectTransform::from_to(
                //Making a rectangle, the same size as the active area
                //But with 0,0 at the center
                Rect::from_min_max(
                    (-1.0 * response.rect.size() / 2.0).to_pos2(),
                    (response.rect.size() / 2.0).to_pos2(),
                ),
                response.rect,
            );
            //Adding zoom behaviour on Ctrl+Mouse Wheel
            if let Some(pointer) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if response.hovered() {
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());
                    if zoom_delta != 1.0 {
                        self.view_scale = self.view_scale * (3.0 + zoom_delta) / 4.0;
                        if zoom_delta < 1.0 {
                            self.drag_distance -= (pointer - response.rect.center())
                                * (1.0 - (3.0 + zoom_delta) / 4.0);
                        } else {
                            self.drag_distance += (pointer - response.rect.center())
                                * (1.0 - (3.0 + zoom_delta) / 4.0);
                        }
                    }
                }
            }

            //Pushing the line shapes to be drawn
            let line_stroke = Stroke::new(2.0, Color32::WHITE);
            let mut title_lines: Vec<Shape> = Vec::new();
            for (title_index_1, title_index_2) in self.linked_pairs.clone() {
                let temp_array: [Pos2; 2] = [
                    to_screen * self.titles[title_index_1].node_screen_position,
                    to_screen * self.titles[title_index_2].node_screen_position,
                ];
                title_lines.push(Shape::line_segment(temp_array, line_stroke.clone()));
            }
            painter.extend(title_lines);
            let half_x: f32 = 50.0 * self.view_scale;
            let half_y: f32 = 15.0 * self.view_scale;
            let mut title_node_shapes: Vec<Shape> = Vec::new();
            //Temp value to store current title in case a node is clicked and the title needs to be
            //changeed
            let mut temp_curr_title = Title::default();
            match self.current_state {
                StateType::Title => temp_curr_title = self.titles[self.current_title_index].clone(),
                _ => (),
            }
            for (index, title) in self.titles.iter_mut().enumerate() {
                let point_in_screen = to_screen.transform_pos(title.node_screen_position);
                //If the point should be visible draw it
                if point_in_screen.x < response.rect.max.x
                    && point_in_screen.x > response.rect.min.x
                    && point_in_screen.y < response.rect.max.y
                    && point_in_screen.y > response.rect.min.y
                {
                    let first_point: Pos2 =
                        (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                    let second_point: Pos2 =
                        (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                    let mut point_rect = Rect::from_two_pos(first_point, second_point);

                    //Adding the image if there is one available
                    if title.image.path.len() > 0 {
                        let file_path = title.image.path.clone();
                        let image = egui::Image::new(format!("file://{file_path}"));
                        let image_size = image
                            .load_and_calc_size(
                                ui,
                                Vec2::new(2.0 * half_x, 1000.0 * self.view_scale),
                            )
                            .unwrap_or(Vec2::new(2.0 * half_x, 100.0 * self.view_scale));
                        //Creating the area for the image
                        //+1.0 Removes a pixel gap
                        let first_point: Pos2 = (
                            point_in_screen.x - half_x,
                            point_in_screen.y - half_y - image_size.y + 1.0,
                        )
                            .into();
                        let mut second_point: Pos2 =
                            (point_in_screen.x + half_x, point_in_screen.y - half_y + 1.0).into();
                        point_rect = Rect::from_two_pos(first_point, second_point);
                        image.paint_at(ui, point_rect);
                        //Drawing the rectangle again so the interactable area contains the button
                        second_point =
                            (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                        point_rect = Rect::from_two_pos(first_point, second_point);
                    }

                    //Getting the drag interaction and updating the point
                    let point_id = response.id.with(index);
                    let point_response_drag = ui.interact(point_rect, point_id, Sense::drag());
                    if point_response_drag.dragged() {
                        title.node_currnetly_clicked = true;
                        title.node_physics_position +=
                            point_response_drag.drag_delta() / self.view_scale;
                    }
                    let point_in_screen = to_screen.transform_pos(title.node_screen_position);
                    //Colouring the button
                    let rect_color = ui.style().interact(&point_response_drag).bg_fill;
                    //Adding the click interaction
                    let point_response_click = ui.interact(point_rect, point_id, Sense::click());

                    if point_response_click.clicked() {
                        match self.current_state {
                            StateType::Title => {
                                save_title(self.project_directory.clone(), temp_curr_title.clone());
                            }
                            _ => (),
                        }
                        self.current_state = StateType::Title;
                        if self.center_current_node {
                            self.drag_distance =
                                -1.0 * title.node_physics_position * self.view_scale;
                        }
                        //Saving the title of the curent page before switching
                        for id in self.current_point_ids.clone() {
                            save_point(self.project_directory.clone(), self.points[&id].clone());
                        }
                        self.current_point_ids = title.point_ids.clone();
                        self.next_page_point_ids = Vec::new();
                        self.current_title_index = index;
                        title.links =
                            title_is_linked_with(self.project_directory.clone(), title.id.clone());
                    }
                    //Creating the rectangle to add it to painter
                    //It has to be calculated again as the previous one is needed for the interaction
                    //response of rect_color
                    let first_point: Pos2 =
                        (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                    let second_point: Pos2 =
                        (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                    let rect_from_point = Rect::from_two_pos(first_point, second_point);
                    title_node_shapes.push(Shape::rect_filled(
                        rect_from_point,
                        Rounding::ZERO,
                        rect_color,
                    ));
                    //Adding text to each button
                    ui.fonts(|f| {
                        let text_on_node: String;
                        if title.name.clone().len() >= 16 {
                            text_on_node = title.name.clone()[..13].to_string() + "...";
                        } else {
                            text_on_node = title.name.clone();
                        }
                        title_node_shapes.push(Shape::text(
                            f,
                            point_in_screen,
                            egui::Align2::CENTER_CENTER,
                            text_on_node,
                            FontId::monospace(10.0 * self.view_scale),
                            Color32::WHITE,
                        ))
                    })
                }
            }
            //Calculate the new node positions
            if self.node_view_start_stop_physics {
                self.node_physics();
            }
            for title in self.titles.iter_mut() {
                title.node_screen_position =
                    (title.node_physics_position * self.view_scale + self.drag_distance).to_pos2();
            }
            painter.extend(title_node_shapes);
        });
    }
}
