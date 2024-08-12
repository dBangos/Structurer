use crate::Structurer;
use eframe::egui::{self};
impl Structurer<'_> {
    pub fn node_view_popup(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Node View")
                .with_inner_size([500.0, 800.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    self.node_view(ui);
                    ui.ctx().request_repaint();
                    if ui.ctx().input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_node_view_popup = false;
                    }
                });
            },
        );
    }
}
