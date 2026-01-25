use serde_json::Value;

pub mod analysis;
pub mod facets;

pub type BiblioRecord = Value;

#[derive(Debug, Clone)]
pub struct RecordIssue {
    pub record_index: usize,
    pub issue_type: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: String,
    pub sample_count: usize,
    pub null_count: usize,
}

#[derive(Debug, Clone)]
pub struct FacetValue {
    pub value: String,
    pub count: usize,
    pub percentage: f32,
}

#[derive(Debug, Clone)]
pub struct FacetAnalysis {
    pub field_name: String,
    pub total_values: usize,
    pub unique_values: usize,
    pub null_count: usize,
    pub values: Vec<FacetValue>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Browse,
    Statistics,
    Issues,
    Details,
    Schema,
    Facets,
}

pub fn get_value_type(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "array (empty)".to_string()
            } else {
                format!("array of {}", get_value_type(&arr[0]))
            }
        }
        Value::Object(_) => "object".to_string(),
    }
}

pub fn extract_searchable_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Array(arr) => {
            arr.iter()
                .map(|v| extract_searchable_text(v))
                .collect::<Vec<_>>()
                .join(" ")
        }
        Value::Object(obj) => {
            obj.values()
                .map(|v| extract_searchable_text(v))
                .collect::<Vec<_>>()
                .join(" ")
        }
        Value::Null => String::new(),
    }
}

pub fn get_display_value(value: &Value, max_len: usize) -> String {
    let display = match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        Value::Array(arr) => {
            if arr.is_empty() {
                "[]".to_string()
            } else {
                format!("[{} items]", arr.len())
            }
        }
        Value::Object(_) => "{...}".to_string(),
    };

    if display.chars().count() > max_len {
        let truncated: String = display.chars().take(max_len).collect();
        format!("{}...", truncated)
    } else {
        display
    }
}