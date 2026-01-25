use eframe::egui;
use crate::app::BiblioAnalyzerApp;
use crate::data::{Tab, get_display_value};

pub fn render_browse_tab(app: &mut BiblioAnalyzerApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("🔎 Search:");
        let response = ui.text_edit_singleline(&mut app.search_query);
        if response.changed() {
            app.apply_filter();
        }
        ui.label(egui::RichText::new(format!("({} records)", app.filtered_records.len()))
            .color(egui::Color32::from_rgb(200, 160, 100)));
    });

    let total_records = app.filtered_records.len();
    let total_pages = (total_records + app.page_size - 1).max(1) / app.page_size;

    ui.add_space(5.0);
    ui.horizontal(|ui| {
        ui.label("Page size:");
        ui.add(egui::DragValue::new(&mut app.page_size).range(10..=1000));

        ui.separator();

        if ui.button("⏮ First").clicked() {
            app.page = 0;
        }
        if ui.button("◀ Prev").clicked() && app.page > 0 {
            app.page -= 1;
        }

        ui.label(format!("Page {} of {}", app.page + 1, total_pages));

        if ui.button("Next ▶").clicked() && app.page < total_pages.saturating_sub(1) {
            app.page += 1;
        }
        if ui.button("Last ⏭").clicked() {
            app.page = total_pages.saturating_sub(1);
        }

        ui.separator();

        let start = app.page * app.page_size;
        let end = (start + app.page_size).min(total_records);
        ui.label(format!("Showing {}-{} of {}", start + 1, end, total_records));
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        let display_fields: Vec<_> = app.top_level_fields.iter().take(5).cloned().collect();

        // Header row
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Index").strong());
            for field in &display_fields {
                ui.label(egui::RichText::new(field).strong());
            }
        });

        ui.separator();

        let start = app.page * app.page_size;
        let end = (start + app.page_size).min(app.filtered_records.len());

        for &idx in &app.filtered_records[start..end] {
            let record = &app.records[idx];
            let is_selected = app.selected_record == Some(idx);

            let response = ui.horizontal(|ui| {
                if is_selected {
                    let rect = ui.available_rect_before_wrap();
                    ui.painter().rect_filled(
                        rect,
                        4.0,
                        egui::Color32::from_rgb(120, 100, 70),
                    );
                }

                ui.label(format!("#{}", idx + 1));

                if let serde_json::Value::Object(obj) = record {
                    for field in &display_fields {
                        let value = obj.get(field).unwrap_or(&serde_json::Value::Null);
                        ui.label(get_display_value(value, 50));
                    }
                }
            }).response;

            if response.interact(egui::Sense::click()).clicked() {
                app.selected_record = Some(idx);
                app.current_tab = Tab::Details;
            }

            ui.add_space(2.0);
        }
    });
}