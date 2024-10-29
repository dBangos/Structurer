use crate::Structurer;
use eframe::egui;
use egui::{vec2, Slider};

impl Structurer {
    pub fn node_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("▶ Resume").clicked() {
                self.node_view_controls.node_view_start_stop_physics = true;
            }
            if ui.button("⏸ Pause").clicked() {
                self.node_view_controls.node_view_start_stop_physics = false;
            }
            if ui.button("▣ Pop Out").clicked() {
                self.show_node_view_popup = !self.show_node_view_popup;
            }
        });
        ui.checkbox(
            &mut self.node_view_controls.stop_clicked_nodes,
            "Hold node into place after interaction",
        );
        ui.checkbox(
            &mut self.node_view_controls.center_current_node,
            "Center current title",
        );
        ui.collapsing("Physics Controls", |ui| {
            ui.horizontal(|ui| {
                ui.label("Gravity: ");
                ui.add(
                    Slider::new(&mut self.node_view_controls.gravity, 0.0..=2.0)
                        .clamping(egui::SliderClamping::Never),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Node Repulsion: ");
                ui.add(
                    Slider::new(&mut self.node_view_controls.node_repulsion, 0.0..=2.0)
                        .clamping(egui::SliderClamping::Never),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Link Pull: ");
                ui.add(
                    Slider::new(&mut self.node_view_controls.link_pull, 0.0..=2.0)
                        .clamping(egui::SliderClamping::Never),
                );
            });
            if ui.button("Reset").clicked() {
                self.node_view_controls.drag_distance = vec2(0.0, 0.0);
                self.node_view_controls.link_pull = 1.0;
                self.node_view_controls.node_repulsion = 1.0;
                self.node_view_controls.gravity = 1.0;
            }
        });
    }
}
