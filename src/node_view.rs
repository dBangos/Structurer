use crate::gui_elements::save_old_add_new_points;
use crate::save_load::all_titles_links;
use crate::Structurer;
use eframe::egui::{self, Pos2};
use egui::emath::TSTransform;
use egui::epaint::PathShape;
use egui::*;

impl Structurer {
    pub fn node_view(&mut self, ui: &mut egui::Ui) {
        Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                Vec2::new(ui.available_width(), ui.available_height()),
                Sense::hover(),
            );

            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            let aux_stroke = Stroke::new(1.0, Color32::RED.linear_multiply(0.25));
            let half_x: f32 = 50.0;
            let half_y: f32 = 15.0;
            let mut title_node_shapes: Vec<Shape> = Vec::new();
            for (index, title_id) in self.title_order.iter_mut().enumerate() {
                //
                let point_in_screen = to_screen.transform_pos(self.titles[title_id].node_position);
                let first_point: Pos2 =
                    (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                let second_point: Pos2 =
                    (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                let point_rect = Rect::from_two_pos(first_point, second_point);
                //Getting the drag interaction and updating the point
                let point_id = response.id.with(index);
                let point_response_1 = ui.interact(point_rect, point_id, Sense::drag());
                self.titles.get_mut(title_id).unwrap().node_position +=
                    point_response_1.drag_delta();
                self.titles.get_mut(title_id).unwrap().node_position =
                    to_screen.from().clamp(self.titles[title_id].node_position);
                let point_in_screen = to_screen.transform_pos(self.titles[title_id].node_position);
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
            }
            let mut points_in_screen: Vec<Pos2> = Vec::new();
            for title_id in &self.title_order {
                points_in_screen.push(to_screen * self.titles[title_id].node_position);
            }
            let mut titles_text: Vec<Shape> = Vec::new();
            for (title_id, point_in_screen) in self
                .title_order
                .clone()
                .into_iter()
                .zip(points_in_screen.clone())
            {
                ui.fonts(|f| {
                    titles_text.push(Shape::text(
                        f,
                        point_in_screen,
                        egui::Align2::CENTER_CENTER,
                        self.titles[&title_id].name.clone(),
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    ))
                })
            }
            let title_link_pairs = self.get_linked_pairs();
            let mut title_lines: Vec<Shape> = Vec::new();
            for (title_1, title_2) in title_link_pairs {
                println!("{} {}", title_1, title_2);
                let temp_array: [Pos2; 2] = [
                    self.titles[&title_1].node_position,
                    self.titles[&title_2].node_position,
                ];
                title_lines.push(Shape::LineSegment {
                    points: (temp_array),
                    stroke: (aux_stroke.into()),
                })
            }
            painter.add(PathShape::line(points_in_screen, aux_stroke));
            painter.extend(title_node_shapes);
            painter.extend(titles_text);
            painter.extend(title_lines);
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
