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

            let mut title_nodes: Vec<Pos2> = Vec::new();
            let aux_stroke = Stroke::new(1.0, Color32::RED.linear_multiply(0.25));
            let mut x = 0.0;
            let mut y = 0.0;
            for title_id in self.title_ids.clone() {
                x += 50.0;
                y += 50.0;
                title_nodes.push(pos2(x, y));
            }
            let title_node_shapes: Vec<Shape> = title_nodes
                .iter_mut()
                .enumerate()
                .map(|(i, point)| {
                    let half_x: f32 = 50.0;
                    let half_y: f32 = 15.0;
                    //Setting the activation area to be the same as the drawn button
                    let point_in_screen = to_screen.transform_pos(*point);
                    let first_point: Pos2 =
                        (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                    let second_point: Pos2 =
                        (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                    let point_rect = Rect::from_two_pos(first_point, second_point);
                    //Getting the drag interaction and updating the point
                    let point_id = response.id.with(i);
                    let point_response_1 = ui.interact(point_rect, point_id, Sense::drag());
                    *point += point_response_1.drag_delta();
                    *point = to_screen.from().clamp(*point);
                    let point_in_screen = to_screen.transform_pos(*point);
                    //Colouring the button
                    let rect_color = ui.style().interact(&point_response_1).bg_fill;
                    //Adding the click interaction
                    let point_response_2 = ui.interact(point_rect, point_id, Sense::click());

                    if point_response_2.clicked() {
                        self.save_old_add_new_points(
                            self.titles[i].clone(),
                            self.points_of_title[i].clone(),
                            self.title_ids[i].clone(),
                        );
                    }
                    //Updating the button after it has been dragged
                    let first_point: Pos2 =
                        (point_in_screen.x - half_x, point_in_screen.y - half_y).into();
                    let second_point: Pos2 =
                        (point_in_screen.x + half_x, point_in_screen.y + half_y).into();
                    let rect_from_point = Rect::from_two_pos(first_point, second_point);
                    Shape::rect_filled(rect_from_point, Rounding::ZERO, rect_color)
                })
                .collect();

            let points_in_screen: Vec<Pos2> = title_nodes.iter().map(|p| to_screen * *p).collect();
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
                        title,
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
