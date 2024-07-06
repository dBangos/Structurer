use crate::gui_elements::save_old_add_new_points;
use crate::save_load::all_titles_links;
use crate::Structurer;
use eframe::egui::{self, Pos2};
use egui::emath::TSTransform;
use egui::epaint::PathShape;
use egui::*;
use std::borrow::BorrowMut;

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
            for (index, title) in self.titles.iter_mut().enumerate() {
                //
                let point_in_screen = to_screen.transform_pos(title.node_position);
                let first_point: Pos2 =
                    (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                let second_point: Pos2 =
                    (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                let point_rect = Rect::from_two_pos(first_point, second_point);
                //Getting the drag interaction and updating the point
                let point_id = response.id.with(index);
                let point_response_1 = ui.interact(point_rect, point_id, Sense::drag());
                title.node_position += point_response_1.drag_delta();
                title.node_position = to_screen.from().clamp(title.node_position);
                let point_in_screen = to_screen.transform_pos(title.node_position);
                //Colouring the button
                let rect_color = ui.style().interact(&point_response_1).bg_fill;
                //Adding the click interaction
                let point_response_2 = ui.interact(point_rect, point_id, Sense::click());
                if point_response_2.clicked() {
                    (self.current_title, self.current_points) = save_old_add_new_points(
                        self.project_directory.clone(),
                        self.current_title.clone(),
                        self.current_points.clone(),
                        title.clone(),
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
            for title in &self.titles {
                points_in_screen.push(to_screen * title.node_position);
            }
            //let points_in_screen: Vec<Pos2> = title_nodes.iter().map(|p| to_screen * *p).collect();
            let mut titles_text: Vec<Shape> = Vec::new();
            for (title, point_in_screen) in self
                .titles
                .clone()
                .into_iter()
                .zip(points_in_screen.clone())
            {
                ui.fonts(|f| {
                    titles_text.push(Shape::text(
                        f,
                        point_in_screen,
                        egui::Align2::CENTER_CENTER,
                        title.name,
                        FontId::monospace(10.0),
                        Color32::WHITE,
                    ))
                })
            }
            painter.add(PathShape::line(points_in_screen, aux_stroke));
            painter.extend(title_node_shapes);
            painter.extend(titles_text);

            response
        });
    }

    //Returns a tuple:
    //First element is a vector of all titles in insertion order and their positions
    //Second element is a vector of all Pos2 pairs for the linking lines
    pub fn get_node_positions(
        &mut self,
    ) -> (Vec<(String, egui::Pos2)>, Vec<(egui::Pos2, egui::Pos2)>) {
        let completed: Vec<String> = Vec::new();
        let titles_positions: Vec<(String, egui::Pos2)> = Vec::new();
        let line_positions: Vec<(egui::Pos2, egui::Pos2)> = Vec::new();
        let links = all_titles_links(self.project_directory.clone());
        for (title_id, links_to_title_id) in links {
            if !completed.contains(&title_id) && links_to_title_id.len() > 0 {
                //
            }
        }
        return (titles_positions, line_positions);
    }
}
