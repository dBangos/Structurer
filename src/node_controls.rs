use crate::Structurer;
use eframe::egui;

impl Structurer {
    pub fn node_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("▶ Resume").clicked() {
                self.node_view_start_stop_physics = true;
            }
            if ui.button("⏸ Pause").clicked() {
                self.node_view_start_stop_physics = false;
            }
        });
        ui.checkbox(
            &mut self.stop_clicked_nodes,
            "Hold node into place after interaction",
        );
    }
}
