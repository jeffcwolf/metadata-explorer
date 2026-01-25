use serde_json::Value;
use std::collections::HashMap;
use super::{BiblioRecord, FacetAnalysis, FacetValue, extract_searchable_text};

pub fn analyze_field_facets(records: &[BiblioRecord], field_name: &str) -> FacetAnalysis {
    let mut value_counts: HashMap<String, usize> = HashMap::new();
    let mut null_count = 0;
    let mut total_values = 0;

    for record in records {
        if let Value::Object(obj) = record {
            if let Some(field_value) = obj.get(field_name) {
                total_values += 1;
                
                if field_value.is_null() {
                    null_count += 1;
                } else {
                    // Extract text representation for counting
                    let text = extract_field_text(field_value);
                    *value_counts.entry(text).or_insert(0) += 1;
                }
            }
        }
    }

    // Convert to sorted facet values
    let mut values: Vec<FacetValue> = value_counts
        .into_iter()
        .map(|(value, count)| {
            let percentage = if total_values > 0 {
                (count as f32 / total_values as f32) * 100.0
            } else {
                0.0
            };
            FacetValue {
                value,
                count,
                percentage,
            }
        })
        .collect();

    // Sort by count descending
    values.sort_by(|a, b| b.count.cmp(&a.count));

    FacetAnalysis {
        field_name: field_name.to_string(),
        total_values,
        unique_values: values.len(),
        null_count,
        values,
    }
}

fn extract_field_text(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "(null)".to_string(),
        Value::Array(arr) => {
            // For arrays, create a combined representation
            if arr.is_empty() {
                "(empty array)".to_string()
            } else {
                // Extract first few items for display
                let items: Vec<String> = arr.iter()
                    .take(3)
                    .map(|v| extract_searchable_text(v))
                    .collect();
                if arr.len() > 3 {
                    format!("{} ... ({} items)", items.join(", "), arr.len())
                } else {
                    items.join(", ")
                }
            }
        }
        Value::Object(_) => "(object)".to_string(),
    }
}