use eframe::egui;
use crate::app::BiblioAnalyzerApp;

pub fn render_schema_tab(app: &BiblioAnalyzerApp, ui: &mut egui::Ui) {
    ui.heading("📋 Data Schema Analysis");
    ui.label(format!("Detected {} unique fields across {} records",
        app.field_schema.len(), app.records.len()));
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("schema_grid")
            .striped(true)
            .min_col_width(150.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Field Name").strong());
                ui.label(egui::RichText::new("Data Type").strong());
                ui.label(egui::RichText::new("Present In").strong());
                ui.label(egui::RichText::new("Null Count").strong());
                ui.label(egui::RichText::new("Coverage %").strong());
                ui.end_row();

                for field in &app.field_schema {
                    let total = app.records.len();
                    let coverage = if total > 0 {
                        field.sample_count as f32 / total as f32 * 100.0
                    } else {
                        0.0
                    };

                    ui.label(&field.name);
                    ui.label(&field.field_type);
                    ui.label(format!("{} / {}", field.sample_count, total));
                    ui.label(format!("{}", field.null_count));

                    let color = if coverage > 90.0 {
                        egui::Color32::from_rgb(100, 200, 100)
                    } else if coverage > 50.0 {
                        egui::Color32::from_rgb(200, 200, 100)
                    } else {
                        egui::Color32::from_rgb(200, 100, 100)
                    };
                    ui.colored_label(color, format!("{:.1}%", coverage));
                    ui.end_row();
                }
            });
    });
}

pub fn render_statistics_tab(app: &BiblioAnalyzerApp, ui: &mut egui::Ui) {
    ui.heading("📊 Dataset Statistics");
    ui.separator();

    ui.label(format!("Total Records: {}", app.records.len()));
    ui.label(format!("Total Fields Detected: {}", app.field_schema.len()));

    ui.add_space(10.0);
    ui.heading("Field Coverage");

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("field_stats")
            .striped(true)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Field Name").strong());
                ui.label(egui::RichText::new("Type").strong());
                ui.label(egui::RichText::new("Present").strong());
                ui.label(egui::RichText::new("Null").strong());
                ui.label(egui::RichText::new("Coverage").strong());
                ui.end_row();

                for field in &app.field_schema {
                    let total = app.records.len();
                    let coverage = if total > 0 {
                        field.sample_count as f32 / total as f32 * 100.0
                    } else {
                        0.0
                    };

                    ui.label(&field.name);
                    ui.label(&field.field_type);
                    ui.label(format!("{}", field.sample_count));
                    ui.label(format!("{}", field.null_count));
                    ui.label(format!("{:.1}%", coverage));
                    ui.end_row();
                }
            });
    });
}

pub fn render_issues_tab(app: &BiblioAnalyzerApp, ui: &mut egui::Ui) {
    ui.heading("⚠️ Data Quality Issues");
    ui.separator();

    ui.label(format!("Total Issues Found: {}", app.issues.len()));
    ui.add_space(10.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("issues_grid")
            .striped(true)
            .min_col_width(100.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("Record").strong());
                ui.label(egui::RichText::new("Issue Type").strong());
                ui.label(egui::RichText::new("Description").strong());
                ui.end_row();

                for issue in &app.issues {
                    let record_label = if issue.record_index == usize::MAX {
                        "SYSTEM".to_string()
                    } else {
                        format!("#{}", issue.record_index + 1)
                    };
                    ui.label(record_label);
                    ui.label(&issue.issue_type);
                    ui.label(&issue.description);
                    ui.end_row();
                }
            });
    });
}