use crate::{PopupActive, Structurer};
use chrono::{NaiveDate, NaiveTime};
use eframe::egui::{self};
use egui::DragValue;
impl Structurer {
    pub fn point_datetime_popup(&mut self, ctx: &egui::Context) {
        if self.popup_active == PopupActive::PointDateTime {
            //Local bool to use for .open() so X in top right corner can be used
            let mut show_popup = true;
            egui::Window::new("Add Date and Time")
                .resizable(false)
                .default_pos([900.0, 400.0])
                .fixed_size([200.0, 200.0])
                .open(&mut show_popup)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_space(23.0);
                                ui.label("Year");
                            });
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(12.0);

                                    if ui.button("⏶").clicked() {
                                        self.point_popup_fields.0 += 1;
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.0));
                                ui.horizontal(|ui| {
                                    ui.add_space(12.0);

                                    if ui.button("⏷").clicked() {
                                        self.point_popup_fields.0 -= 1;
                                    }
                                });
                            });
                            ui.separator();
                            ui.label("Month");
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏶").clicked() {
                                        if self.point_popup_fields.1 < 12 {
                                            self.point_popup_fields.1 += 1;
                                        }
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.1));
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏷").clicked() {
                                        if self.point_popup_fields.1 > 1 {
                                            self.point_popup_fields.1 -= 1;
                                        }
                                    }
                                });
                                if self.point_popup_fields.1 > 12 {
                                    self.point_popup_fields.1 = 12;
                                } else if self.point_popup_fields.1 < 1 {
                                    self.point_popup_fields.1 = 1;
                                }
                            });
                            ui.separator();
                            ui.label("Day");
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏶").clicked() {
                                        if self.point_popup_fields.2 < 31 {
                                            self.point_popup_fields.2 += 1;
                                        }
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.2));
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏷").clicked() {
                                        if self.point_popup_fields.2 > 1 {
                                            self.point_popup_fields.2 -= 1;
                                        }
                                    }
                                });
                                if self.point_popup_fields.2 > 31 {
                                    self.point_popup_fields.2 = 31;
                                } else if self.point_popup_fields.2 < 1 {
                                    self.point_popup_fields.2 = 1;
                                }
                            });
                            ui.separator();

                            ui.label("Hour");
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏶").clicked() {
                                        if self.point_popup_fields.3 < 23 {
                                            self.point_popup_fields.3 += 1;
                                        }
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.3));
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏷").clicked() {
                                        if self.point_popup_fields.3 > 0 {
                                            self.point_popup_fields.3 -= 1;
                                        }
                                    }
                                });
                                if self.point_popup_fields.3 > 23 {
                                    self.point_popup_fields.3 = 23;
                                }
                            });
                            ui.separator();
                            ui.label("Minute");
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏶").clicked() {
                                        if self.point_popup_fields.4 < 59 {
                                            self.point_popup_fields.4 += 1;
                                        }
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.4));
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏷").clicked() {
                                        if self.point_popup_fields.4 > 0 {
                                            self.point_popup_fields.4 -= 1;
                                        }
                                    }
                                });
                                if self.point_popup_fields.4 > 59 {
                                    self.point_popup_fields.4 = 59;
                                }
                            });
                            ui.separator();
                            ui.label("Second");
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏶").clicked() {
                                        if self.point_popup_fields.5 < 59 {
                                            self.point_popup_fields.5 += 1;
                                        }
                                    }
                                });
                                ui.add(DragValue::new(&mut self.point_popup_fields.5));
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);

                                    if ui.button("⏷").clicked() {
                                        if self.point_popup_fields.5 > 0 {
                                            self.point_popup_fields.5 -= 1;
                                        }
                                    }
                                });
                                if self.point_popup_fields.5 > 59 {
                                    self.point_popup_fields.5 = 59;
                                }
                            });
                        });
                        ui.separator();
                        if NaiveDate::from_ymd_opt(
                            self.point_popup_fields.0,
                            self.point_popup_fields.1,
                            self.point_popup_fields.2,
                        )
                        .is_none()
                        {
                            let mut temp_day = self.point_popup_fields.2.clone();
                            while NaiveDate::from_ymd_opt(
                                self.point_popup_fields.0,
                                self.point_popup_fields.1,
                                temp_day,
                            )
                            .is_none()
                            {
                                temp_day -= 1;
                            }
                            let temp_string = format!(
                                "The date entered is not valid. Did you mean to enter {}/{}/{}?",
                                self.point_popup_fields.0, self.point_popup_fields.1, temp_day
                            );
                            ui.label(temp_string);
                        }
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            if ui.button("✅ Ok").clicked() {
                                if let Some(temp_date) = NaiveDate::from_ymd_opt(
                                    self.point_popup_fields.0,
                                    self.point_popup_fields.1,
                                    self.point_popup_fields.2,
                                ) {
                                    if let Some(temp_time) = NaiveTime::from_hms_opt(
                                        self.point_popup_fields.3,
                                        self.point_popup_fields.4,
                                        self.point_popup_fields.5,
                                    ) {
                                        self.popup_active = PopupActive::Empty;
                                        self.points
                                            .get_mut(&self.point_requesting_action_id)
                                            .unwrap()
                                            .date = Some(temp_date);
                                        self.points
                                            .get_mut(&self.point_requesting_action_id)
                                            .unwrap()
                                            .time = Some(temp_time);
                                    }
                                }
                            }
                            if ui.button("✖ Close").clicked() {
                                self.popup_active = PopupActive::Empty;
                            }
                        });
                    });
                });
            if !show_popup {
                self.popup_active = PopupActive::Empty;
            }
        }
    }
}
