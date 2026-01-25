use eframe::egui;
use crate::data::{FacetAnalysis, BiblioRecord};
use crate::app::FacetsState;

pub fn render_facets_tab(
    ui: &mut egui::Ui,
    state: &mut FacetsState,
    records: &[BiblioRecord],
    available_fields: &[String],
) {
    ui.heading("🔍 Facet Analysis");
    ui.label("Explore the distribution of values in any field");
    ui.separator();

    // Field selector
    ui.horizontal(|ui| {
        ui.label("Select field to analyze:");
        egui::ComboBox::from_label("")
            .selected_text(state.selected_field.as_deref().unwrap_or("Choose a field..."))
            .show_ui(ui, |ui| {
                for field in available_fields {
                    if ui.selectable_label(
                        state.selected_field.as_ref() == Some(field),
                        field
                    ).clicked() {
                        state.analyze_field(records, field);
                    }
                }
            });
    });

    ui.add_space(10.0);

    // Show analysis if field is selected
    if let Some(analysis) = &state.current_analysis {
        // Summary section with visual styling
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.heading(format!("Field: {}", analysis.field_name));
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("📊 Total Records:")
                        .color(egui::Color32::from_rgb(200, 160, 100)));
                    ui.label(format!("{}", analysis.total_values));
                });
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✨ Unique Values:")
                        .color(egui::Color32::from_rgb(200, 160, 100)));
                    ui.label(format!("{}", analysis.unique_values));
                });
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("❌ Null Values:")
                        .color(egui::Color32::from_rgb(200, 160, 100)));
                    ui.label(format!("{}", analysis.null_count));
                });
            });
        });

        ui.add_space(15.0);
        ui.heading("Value Distribution");
        ui.separator();

        // Show values table
        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                egui::Grid::new("facet_values")
                    .striped(true)
                    .min_col_width(100.0)
                    .show(ui, |ui| {
                        // Header
                        ui.label(egui::RichText::new("Value").strong());
                        ui.label(egui::RichText::new("Count").strong());
                        ui.label(egui::RichText::new("Percentage").strong());
                        ui.label(egui::RichText::new("Bar").strong());
                        ui.end_row();

                        // Values
                        for facet_value in &analysis.values {
                            // Value (truncated if too long)
                            let display_value = if facet_value.value.len() > 60 {
                                format!("{}...", &facet_value.value[..60])
                            } else {
                                facet_value.value.clone()
                            };
                            ui.label(display_value);

                            // Count
                            ui.label(format!("{}", facet_value.count));

                            // Percentage
                            ui.label(format!("{:.1}%", facet_value.percentage));

                            // Visual bar
                            let bar_width = (facet_value.percentage / 100.0) * 200.0;
                            let (rect, _response) = ui.allocate_exact_size(
                                egui::vec2(200.0, 16.0),
                                egui::Sense::hover(),
                            );
                            
                            // Background
                            ui.painter().rect_filled(
                                rect,
                                2.0,
                                egui::Color32::from_rgb(65, 55, 43),
                            );
                            
                            // Filled portion
                            if bar_width > 0.0 {
                                let bar_rect = egui::Rect::from_min_size(
                                    rect.min,
                                    egui::vec2(bar_width, 16.0),
                                );
                                ui.painter().rect_filled(
                                    bar_rect,
                                    2.0,
                                    egui::Color32::from_rgb(200, 160, 100),
                                );
                            }

                            ui.end_row();
                        }
                    });
            });
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("👆 Select a field above to begin analysis");
            ui.label("The facet view will show all unique values and their distribution");
        });
    }
}