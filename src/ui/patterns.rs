use eframe::egui;
use crate::data::PatternAnalysis;
use crate::app::{PatternsState, FacetsState};

pub fn render_patterns_tab(
    ui: &mut egui::Ui,
    patterns_state: &mut PatternsState,
    facets_state: &FacetsState,
) {
    ui.heading("🔍 Pattern Analysis");
    ui.label("Automatic detection of data patterns and formats");
    ui.separator();

    // Check if we have facet analysis to work from
    if let Some(facet_analysis) = &facets_state.current_analysis {
        ui.horizontal(|ui| {
            ui.label(format!("📊 Analyzing patterns in field: {}", facet_analysis.field_name));
            
            if ui.button("🔄 Analyze Patterns").clicked() {
                patterns_state.analyze_from_facets(facet_analysis);
            }
        });

        ui.add_space(10.0);

        // Show pattern analysis if available
        if let Some(analysis) = &patterns_state.current_analysis {
            render_pattern_results(ui, analysis);
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.heading("👆 Click 'Analyze Patterns' to detect data patterns");
                ui.label("Pattern detection will categorize values by format and structure");
            });
        }
    } else {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            ui.heading("📊 No field selected");
            ui.label("Go to the Facets tab and select a field to analyze");
            ui.add_space(10.0);
            ui.label("Pattern analysis detects:");
            ui.label("• Date formats (ISO dates, 4-digit years, fuzzy dates)");
            ui.label("• Data types (numeric, text, mixed)");
            ui.label("• Bibliographic patterns (language codes, bracketed content)");
            ui.label("• Data quality issues (empty values, malformed data)");
        });
    }
}

fn render_pattern_results(ui: &mut egui::Ui, analysis: &PatternAnalysis) {
    // Summary section
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.heading(format!("Field: {}", analysis.field_name));
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("📊 Total Values:")
                    .color(egui::Color32::from_rgb(200, 160, 100)));
                ui.label(format!("{}", analysis.total_values));
            });
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🎯 Pattern Types Found:")
                    .color(egui::Color32::from_rgb(200, 160, 100)));
                ui.label(format!("{}", analysis.pattern_groups.len()));
            });
        });
    });

    ui.add_space(15.0);
    ui.heading("Pattern Distribution");
    ui.separator();

    // Pattern groups table
    egui::ScrollArea::vertical()
        .max_height(600.0)
        .show(ui, |ui| {
            for pattern_group in &analysis.pattern_groups {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        // Pattern name and description
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new(pattern_group.pattern_type.name())
                                .strong()
                                .size(16.0)
                                .color(egui::Color32::from_rgb(200, 160, 100)));
                            ui.label(egui::RichText::new(format!("({} values, {:.1}%)", 
                                pattern_group.count, pattern_group.percentage))
                                .color(egui::Color32::from_rgb(180, 180, 180)));
                        });

                        ui.label(egui::RichText::new(pattern_group.pattern_type.description())
                            .italics()
                            .color(egui::Color32::from_rgb(200, 190, 170)));

                        ui.add_space(5.0);

                        // Visual bar
                        let bar_width = (pattern_group.percentage / 100.0) * 400.0;
                        let (rect, _response) = ui.allocate_exact_size(
                            egui::vec2(400.0, 20.0),
                            egui::Sense::hover(),
                        );
                        
                        // Background
                        ui.painter().rect_filled(
                            rect,
                            4.0,
                            egui::Color32::from_rgb(65, 55, 43),
                        );
                        
                        // Filled portion
                        if bar_width > 0.0 {
                            let bar_rect = egui::Rect::from_min_size(
                                rect.min,
                                egui::vec2(bar_width, 20.0),
                            );
                            ui.painter().rect_filled(
                                bar_rect,
                                4.0,
                                egui::Color32::from_rgb(200, 160, 100),
                            );
                        }

                        // Examples
                        if !pattern_group.examples.is_empty() {
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Examples:")
                                    .color(egui::Color32::from_rgb(180, 170, 150)));
                                
                                let examples_text = pattern_group.examples
                                    .iter()
                                    .take(3)
                                    .map(|ex| format!("\"{}\"", truncate_string(ex, 40)))
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                
                                ui.label(egui::RichText::new(examples_text)
                                    .color(egui::Color32::from_rgb(180, 220, 180)));
                            });
                        }
                    });
                });

                ui.add_space(10.0);
            }
        });
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}