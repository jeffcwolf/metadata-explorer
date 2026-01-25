use eframe::egui;
use crate::data::*;
use crate::data::analysis::*;
use crate::prefs::*;

pub struct FacetsState {
    pub selected_field: Option<String>,
    pub current_analysis: Option<FacetAnalysis>,
}

impl FacetsState {
    pub fn new() -> Self {
        Self {
            selected_field: None,
            current_analysis: None,
        }
    }

    pub fn analyze_field(&mut self, records: &[BiblioRecord], field_name: &str) {
        use crate::data::facets::analyze_field_facets;
        self.selected_field = Some(field_name.to_string());
        self.current_analysis = Some(analyze_field_facets(records, field_name));
    }
}

pub struct BiblioAnalyzerApp {
    pub records: Vec<BiblioRecord>,
    pub search_query: String,
    pub filtered_records: Vec<usize>,
    pub selected_record: Option<usize>,
    pub issues: Vec<RecordIssue>,
    pub current_tab: Tab,
    pub file_path: String,
    pub error_message: String,
    pub page: usize,
    pub page_size: usize,
    pub loading: bool,
    pub field_schema: Vec<FieldInfo>,
    pub top_level_fields: Vec<String>,
    pub facets_state: FacetsState,
}

impl Default for BiblioAnalyzerApp {
    fn default() -> Self {
        let default_path = load_last_file_path().unwrap_or_else(|| {
            if std::path::Path::new("sample_data.json").exists() {
                "sample_data.json".to_string()
            } else {
                String::new()
            }
        });

        Self {
            records: Vec::new(),
            search_query: String::new(),
            filtered_records: Vec::new(),
            selected_record: None,
            issues: Vec::new(),
            current_tab: Tab::Browse,
            file_path: default_path,
            error_message: String::new(),
            page: 0,
            page_size: 100,
            loading: false,
            field_schema: Vec::new(),
            top_level_fields: Vec::new(),
            facets_state: FacetsState::new(),
        }
    }
}

impl BiblioAnalyzerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_sepia_theme(&cc.egui_ctx);
        Default::default()
    }

    pub fn load_file(&mut self, path: &str) {
        self.error_message.clear();
        self.loading = true;

        match std::fs::read_to_string(path) {
            Ok(content) => {
                match serde_json::from_str::<Vec<serde_json::Value>>(&content) {
                    Ok(records) => {
                        self.records = records;
                        self.file_path = path.to_string();
                        save_last_file_path(path);
                        self.page = 0;
                        
                        let (schema, fields) = analyze_schema(&self.records);
                        self.field_schema = schema;
                        self.top_level_fields = fields;
                        
                        self.issues = analyze_quality(&self.records);
                        self.apply_filter();
                        self.loading = false;
                    }
                    Err(e) => {
                        self.error_message = format!("Error parsing JSON: {}", e);
                        self.loading = false;
                    }
                }
            }
            Err(e) => {
                self.error_message = format!("Error reading file: {}", e);
                self.loading = false;
            }
        }
    }

    pub fn apply_filter(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_records = if query.is_empty() {
            (0..self.records.len()).collect()
        } else {
            self.records
                .iter()
                .enumerate()
                .filter(|(_, r)| {
                    let searchable = extract_searchable_text(r);
                    searchable.to_lowercase().contains(&query)
                })
                .map(|(i, _)| i)
                .collect()
        };
        self.page = 0;
    }
}

fn setup_sepia_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    let bg_dark = egui::Color32::from_rgb(45, 38, 30);
    let bg_medium = egui::Color32::from_rgb(65, 55, 43);
    let bg_light = egui::Color32::from_rgb(85, 72, 56);
    let text_color = egui::Color32::from_rgb(220, 205, 180);
    let text_strong = egui::Color32::from_rgb(245, 230, 200);
    let accent = egui::Color32::from_rgb(200, 160, 100);
    let selection = egui::Color32::from_rgb(120, 100, 70);

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(text_color);
    style.visuals.hyperlink_color = accent;
    style.visuals.selection.bg_fill = selection;
    style.visuals.selection.stroke.color = accent;

    style.visuals.widgets.noninteractive.bg_fill = bg_medium;
    style.visuals.widgets.noninteractive.fg_stroke.color = text_color;
    style.visuals.widgets.noninteractive.weak_bg_fill = bg_dark;
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);

    style.visuals.widgets.inactive.bg_fill = bg_medium;
    style.visuals.widgets.inactive.fg_stroke.color = text_color;
    style.visuals.widgets.inactive.weak_bg_fill = bg_dark;
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);

    style.visuals.widgets.hovered.bg_fill = bg_light;
    style.visuals.widgets.hovered.fg_stroke.color = text_strong;
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);

    style.visuals.widgets.active.bg_fill = accent;
    style.visuals.widgets.active.fg_stroke.color = bg_dark;
    style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);

    style.visuals.window_fill = bg_dark;
    style.visuals.panel_fill = bg_dark;
    style.visuals.extreme_bg_color = bg_medium;
    style.visuals.faint_bg_color = bg_medium;
    style.visuals.code_bg_color = bg_medium;
    style.visuals.warn_fg_color = egui::Color32::from_rgb(220, 180, 100);
    style.visuals.error_fg_color = egui::Color32::from_rgb(200, 100, 80);
    
    style.visuals.striped = true;
    style.visuals.window_stroke.color = accent;
    style.visuals.window_stroke.width = 1.0;
    style.visuals.window_rounding = egui::Rounding::same(6.0);
    
    // Better spacing
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.spacing.window_margin = egui::Margin::same(10.0);

    ctx.set_style(style);

    let mut visuals = ctx.style().visuals.clone();
    visuals.dark_mode = true;
    visuals.override_text_color = Some(text_color);
    visuals.panel_fill = bg_dark;
    visuals.window_fill = bg_dark;
    ctx.set_visuals(visuals);
}