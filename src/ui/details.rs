use eframe::egui;
use serde_json::Value;
use crate::app::BiblioAnalyzerApp;
use crate::data::get_display_value;

pub fn render_details_tab(app: &BiblioAnalyzerApp, ui: &mut egui::Ui) {
    if let Some(idx) = app.selected_record {
        if idx < app.records.len() {
            let record = &app.records[idx];

            ui.heading(format!("📄 Record #{}", idx + 1));
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut row_num = 0;
                render_value_tree(ui, record, 0, &mut row_num);
            });
        }
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("No record selected");
            ui.label("Select a record from the Browse tab to view its details");
        });
    }
}

fn render_value_tree(ui: &mut egui::Ui, value: &Value, depth: usize, row_num: &mut usize) {
    let indent = (depth as f32) * 20.0;

    match value {
        Value::Object(obj) => {
            for (key, val) in obj {
                *row_num += 1;
                let bg_color = if *row_num % 2 == 0 {
                    egui::Color32::from_rgb(55, 47, 38)
                } else {
                    egui::Color32::from_rgb(45, 38, 30)
                };

                match val {
                    Value::Object(_) => {
                        ui.horizontal(|ui| {
                            ui.add_space(indent);
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 2.0, bg_color);
                            ui.label(egui::RichText::new(format!("{}:", key))
                                .strong()
                                .color(egui::Color32::from_rgb(200, 160, 100)));
                        });
                        render_value_tree(ui, val, depth + 1, row_num);
                    }
                    Value::Array(_) => {
                        ui.horizontal(|ui| {
                            ui.add_space(indent);
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 2.0, bg_color);
                            ui.label(egui::RichText::new(format!("{}:", key))
                                .strong()
                                .color(egui::Color32::from_rgb(200, 160, 100)));
                        });
                        render_value_tree(ui, val, depth + 1, row_num);
                    }
                    _ => {
                        ui.horizontal(|ui| {
                            ui.add_space(indent);
                            let rect = ui.available_rect_before_wrap();
                            ui.painter().rect_filled(rect, 2.0, bg_color);
                            ui.label(egui::RichText::new(format!("{}: ", key))
                                .color(egui::Color32::from_rgb(245, 230, 200)));
                            ui.label(egui::RichText::new(get_display_value(val, 200))
                                .color(egui::Color32::from_rgb(180, 220, 180)));
                        });
                    }
                }
            }
        }
        Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                *row_num += 1;
                let bg_color = if *row_num % 2 == 0 {
                    egui::Color32::from_rgb(55, 47, 38)
                } else {
                    egui::Color32::from_rgb(45, 38, 30)
                };

                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    let rect = ui.available_rect_before_wrap();
                    ui.painter().rect_filled(rect, 2.0, bg_color);
                    ui.label(egui::RichText::new(format!("[{}]:", i))
                        .color(egui::Color32::from_rgb(200, 180, 140)));
                });
                render_value_tree(ui, item, depth + 1, row_num);
            }
        }
        _ => {
            ui.horizontal(|ui| {
                ui.add_space(indent);
                ui.label(get_display_value(value, 200));
            });
        }
    }
}