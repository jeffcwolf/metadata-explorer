pub mod browse;
pub mod details;
pub mod facets;
pub mod patterns;
pub mod other_tabs;

use eframe::egui;
use crate::app::BiblioAnalyzerApp;
use crate::data::Tab;

pub fn render_main_ui(app: &mut BiblioAnalyzerApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("📚 Bibliographic Metadata Analyzer");
        ui.separator();

        // File loading section
        ui.horizontal(|ui| {
            ui.label("JSON File:");
            ui.text_edit_singleline(&mut app.file_path);

            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .add_filter("All files", &["*"])
                    .pick_file()
                {
                    app.file_path = path.display().to_string();
                    app.load_file(&app.file_path.clone());
                }
            }

            let load_button = ui.add_enabled(
                !app.loading && !app.file_path.is_empty(),
                egui::Button::new(if app.loading { "Loading..." } else { "Load File" })
            );

            if load_button.clicked() {
                app.load_file(&app.file_path.clone());
            }
        });

        if !app.error_message.is_empty() {
            ui.colored_label(egui::Color32::from_rgb(200, 100, 80), &app.error_message);
        }

        if app.loading {
            ui.spinner();
            ui.label("Loading and analyzing structure...");
        }

        if !app.records.is_empty() {
            ui.add_space(10.0);

            // Tab selection
            ui.horizontal(|ui| {
                ui.selectable_value(&mut app.current_tab, Tab::Browse, "📋 Browse");
                ui.selectable_value(&mut app.current_tab, Tab::Schema, "🔍 Schema");
                ui.selectable_value(&mut app.current_tab, Tab::Facets, "📊 Facets");
                ui.selectable_value(&mut app.current_tab, Tab::Patterns, "🔎 Patterns");
                ui.selectable_value(&mut app.current_tab, Tab::Statistics, "📈 Statistics");
                ui.selectable_value(&mut app.current_tab, Tab::Issues,
                    format!("⚠ Issues ({})", app.issues.len()));
                ui.selectable_value(&mut app.current_tab, Tab::Details, "📄 Details");
            });

            ui.separator();

            // Tab content
            match app.current_tab {
                Tab::Browse => browse::render_browse_tab(app, ui),
                Tab::Schema => other_tabs::render_schema_tab(app, ui),
                Tab::Facets => facets::render_facets_tab(ui, &mut app.facets_state, &app.records, &app.top_level_fields),
                Tab::Patterns => patterns::render_patterns_tab(ui, &mut app.patterns_state, &app.facets_state),
                Tab::Statistics => other_tabs::render_statistics_tab(app, ui),
                Tab::Issues => other_tabs::render_issues_tab(app, ui),
                Tab::Details => details::render_details_tab(app, ui),
            }
        }
    });
}