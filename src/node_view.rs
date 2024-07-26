use crate::gui_elements::save_old_add_new_points;
use crate::save_load::all_titles_links;
use crate::Structurer;
use eframe::egui::{self, Pos2};
use egui::emath::RectTransform;
use egui::{Color32, FontId, Frame, Rect, Rounding, Sense, Shape, Stroke, Vec2};
impl Structurer {
    pub fn node_view(&mut self, ui: &mut egui::Ui) {
        //Flags for the buttons, kinda ugly but I want the buttons outside the canvas
        //But I need to be inside the canvas to calculate the centering offset
        Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                Vec2::new(ui.available_width(), ui.available_height()),
                Sense::click_and_drag(),
            );
            // Allow dragging the background
            if response.dragged() {
                self.drag_distance = response.drag_delta();
                for title_id in self.title_order.clone() {
                    self.titles.get_mut(&title_id).unwrap().node_position =
                        self.titles[&title_id].node_position + self.drag_distance;
                }
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
                    let pointer_in_layer = to_screen * pointer;
                    let zoom_delta = ui.ctx().input(|i| i.zoom_delta());

                    if zoom_delta != 1.0 {
                        for title_id in self.title_order.clone() {
                            self.titles.get_mut(&title_id).unwrap().node_position =
                                self.titles[&title_id].node_position * zoom_delta;
                            //Adjusting the view scale so the ui scales accordingly
                            //Doing the plus, /2 to make the change slower
                            self.view_scale = self.view_scale * (9.0 + zoom_delta) / 10.0;
                        }
                    }
                }
            }
            let line_stroke = Stroke::new(1.0, Color32::RED);
            let title_link_pairs = self.get_linked_pairs();

            //Pushing the line shapes to be drawn
            let mut title_lines: Vec<Shape> = Vec::new();
            for (title_1, title_2) in title_link_pairs.clone() {
                let temp_array: [Pos2; 2] = [
                    to_screen * self.titles[&title_1].node_position,
                    to_screen * self.titles[&title_2].node_position,
                ];
                title_lines.push(Shape::line_segment(temp_array, line_stroke.clone()));
            }
            painter.extend(title_lines);
            let half_x: f32 = 50.0 * self.view_scale;
            let half_y: f32 = 15.0 * self.view_scale;
            let mut title_node_shapes: Vec<Shape> = Vec::new();
            for (index, title_id) in self.title_order.iter_mut().enumerate() {
                let point_in_screen = to_screen.transform_pos(self.titles[title_id].node_position);
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

                    if self.titles[title_id].image.path.len() > 0 {
                        //Creating the area for the image
                        let first_point: Pos2 = (
                            point_in_screen.x - half_x,
                            point_in_screen.y - half_y - 100.0 * self.view_scale,
                        )
                            .into();
                        let mut second_point: Pos2 =
                            (point_in_screen.x + half_x, point_in_screen.y - half_y).into();
                        point_rect = Rect::from_two_pos(first_point, second_point);
                        let file_path = self.titles[title_id].image.path.clone();
                        let image = egui::Image::new(format!("file://{file_path}"))
                            .paint_at(ui, point_rect);
                        //Drawing the rectangle again so the interactable area contains the button
                        second_point =
                            (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                        point_rect = Rect::from_two_pos(first_point, second_point);
                    }

                    //Getting the drag interaction and updating the point
                    let point_id = response.id.with(index);
                    let point_response_1 = ui.interact(point_rect, point_id, Sense::drag());
                    self.titles.get_mut(title_id).unwrap().node_position +=
                        point_response_1.drag_delta();
                    self.titles.get_mut(title_id).unwrap().node_position =
                        to_screen.from().clamp(self.titles[title_id].node_position);
                    let point_in_screen =
                        to_screen.transform_pos(self.titles[title_id].node_position);
                    //Colouring the button
                    let rect_color = ui.style().interact(&point_response_1).bg_fill;
                    //Adding the click interaction
                    let point_response_2 = ui.interact(point_rect, point_id, Sense::click());
                    if point_response_2.clicked() {
                        (self.current_title, self.current_points) = save_old_add_new_points(
                            self.project_directory.clone(),
                            self.current_title.clone(),
                            self.current_points.clone(),
                            self.titles[title_id].clone(),
                        );
                    }
                    //Updating the button after it has been dragged
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
                        title_node_shapes.push(Shape::text(
                            f,
                            point_in_screen,
                            egui::Align2::CENTER_CENTER,
                            self.titles[title_id].name.clone(),
                            FontId::monospace(10.0 * self.view_scale),
                            Color32::WHITE,
                        ))
                    })
                }
            }
            let edge_length: f32 = 100.0;
            let divider: f32 = 400.0;
            let gravity_constant: f32 = 0.1;
            let force_constant: f32 = 10000.0;
            for title_1 in self.title_order.clone() {
                //Gravity
                self.titles.get_mut(&title_1).unwrap().node_force =
                    self.titles[&title_1].node_position.to_vec2() * (-1.0) * gravity_constant;

                println!("Gravity: {}", self.titles[&title_1].node_force);
                //Repulsive forces
                for title_2 in self.title_order.clone() {
                    if self.titles[&title_1].id == self.titles[&title_2].id {
                        continue;
                    } else {
                        let dir = self.titles[&title_1].node_position
                            - self.titles[&title_2].node_position;
                        let mut repulsive_force: Vec2 = Vec2::new(10.0, 10.0);
                        if dir.length() != 0.0 {
                            repulsive_force = dir / (dir.length() * dir.length()) * force_constant;
                        }
                        self.titles.get_mut(&title_1).unwrap().node_force -= repulsive_force;
                        self.titles.get_mut(&title_2).unwrap().node_force += repulsive_force;
                        println!("Repulsive: {}", self.titles[&title_1].node_force);
                        println!("Repulsive: {}", self.titles[&title_2].node_force);
                    }
                }
            }
            //
            ////Loop pulling in links
            //for (title_1, title_2) in title_link_pairs {
            //    let dir = self.titles[&title_1].node_position - self.titles[&title_2].node_position;
            //
            //    let diff = dir - Vec2::new(edge_length, edge_length);
            //    self.titles.get_mut(&title_1).unwrap().node_force -= diff;
            //    self.titles.get_mut(&title_2).unwrap().node_force += diff;
            //}
            //
            //for title_1 in self.title_order.clone() {
            //    println!("{}", self.titles[&title_1].node_force);
            //
            //    self.titles.get_mut(&title_1).unwrap().node_position =
            //        self.titles[&title_1].node_force.to_pos2() / divider;
            //}
            ////Loop spreading out nodes
            //for title_1 in self.title_order.clone() {
            //    for title_2 in self.title_order.clone() {
            //        if self.titles[&title_1].id == self.titles[&title_2].id {
            //            continue;
            //        } else {
            //            if (self.titles[&title_1].node_position
            //                - self.titles[&title_2].node_position)
            //                .length()
            //                < 100.0
            //            {
            //                self.titles.get_mut(&title_1).unwrap().node_position =
            //                    move_point_in_line(
            //                        self.titles[&title_1].node_position,
            //                        self.titles[&title_2].node_position,
            //                        false,
            //                        10.0,
            //                    );
            //            }
            //        }
            //    }
            //}

            ////Loop pulling in links
            //for (title_1, title_2) in title_link_pairs {
            //    let distance = self.titles[&title_1]
            //        .node_position
            //        .distance(self.titles[&title_2].node_position);
            //    if distance > 300.0 {
            //        let first_point = self.titles[&title_1].node_position;
            //        let second_point = self.titles[&title_2].node_position;
            //        self.titles.get_mut(&title_1).unwrap().node_position =
            //            move_point_in_line(first_point, second_point, true, 2.0);
            //        self.titles.get_mut(&title_1).unwrap().node_position =
            //            move_point_in_line(second_point, first_point, true, 2.0);
            //    }
            //}
            painter.extend(title_node_shapes);
            response
        });
    }

    //Returns a vector with all the linked title pairs by index
    fn get_linked_pairs(&mut self) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();
        let all_links = all_titles_links(self.project_directory.clone());
        for (title, links) in all_links {
            for link in links {
                if !(result.contains(&(title.clone(), link.clone()))
                    || result.contains(&(link.clone(), title.clone())))
                {
                    result.push((title.clone(), link.clone()));
                }
            }
        }
        return result;
    }
}

fn move_point_in_line(
    first_point: Pos2,
    second_point: Pos2,
    closer: bool,
    move_distance_fraction: f32,
) -> Pos2 {
    let mut result: Pos2 = Default::default();
    if closer {
        if first_point.x > second_point.x {
            result.x = first_point.x - (first_point.x - second_point.x) / move_distance_fraction;
        } else if first_point.x == second_point.x {
            result.x = first_point.x;
        } else {
            result.x = first_point.x + (second_point.x - first_point.x) / move_distance_fraction;
        }
        if first_point.y > second_point.y {
            result.y = first_point.y - (first_point.y - second_point.y) / move_distance_fraction;
        } else if first_point.y == second_point.y {
            result.y = first_point.y;
        } else {
            result.y = first_point.y + (second_point.y - first_point.y) / move_distance_fraction;
        }
    } else {
        if first_point.x > second_point.x {
            result.x = first_point.x + (first_point.x - second_point.x) / move_distance_fraction;
        } else if first_point.x == second_point.x {
            result.x = first_point.x;
        } else {
            result.x = first_point.x - (second_point.x - first_point.x) / move_distance_fraction;
        }
        if first_point.y > second_point.y {
            result.y = first_point.y + (first_point.y - second_point.y) / move_distance_fraction;
        } else if first_point.y == second_point.y {
            result.y = first_point.y;
        } else {
            result.y = first_point.y - (second_point.y - first_point.y) / move_distance_fraction;
        }
    }
    return result;
}
