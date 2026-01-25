use serde_json::Value;
use std::collections::HashMap;
use super::{BiblioRecord, RecordIssue, FieldInfo, get_value_type};

pub fn analyze_schema(records: &[BiblioRecord]) -> (Vec<FieldInfo>, Vec<String>) {
    let mut field_schema = Vec::new();
    let mut top_level_fields = Vec::new();

    if records.is_empty() {
        return (field_schema, top_level_fields);
    }

    let mut field_map: HashMap<String, (usize, usize, String)> = HashMap::new();

    for record in records {
        if let Value::Object(obj) = record {
            for (key, value) in obj {
                let entry = field_map.entry(key.clone()).or_insert((0, 0, String::new()));

                if value.is_null() {
                    entry.1 += 1;
                } else {
                    entry.0 += 1;
                    if entry.2.is_empty() {
                        entry.2 = get_value_type(value);
                    }
                }
            }
        }
    }

    for (name, (sample_count, null_count, field_type)) in field_map {
        top_level_fields.push(name.clone());
        field_schema.push(FieldInfo {
            name,
            field_type,
            sample_count,
            null_count,
        });
    }

    top_level_fields.sort();
    field_schema.sort_by(|a, b| a.name.cmp(&b.name));

    (field_schema, top_level_fields)
}

pub fn analyze_quality(records: &[BiblioRecord]) -> Vec<RecordIssue> {
    let mut issues = Vec::new();

    let max_records_to_analyze = 10_000;
    let records_to_check = records.len().min(max_records_to_analyze);
    let is_limited = records.len() > max_records_to_analyze;

    for (idx, record) in records.iter().enumerate().take(records_to_check) {
        if !record.is_object() {
            issues.push(RecordIssue {
                record_index: idx,
                issue_type: "Invalid Structure".to_string(),
                description: "Record is not a JSON object".to_string(),
            });
        }
    }

    if is_limited {
        issues.push(RecordIssue {
            record_index: usize::MAX,
            issue_type: "Analysis Limited".to_string(),
            description: format!(
                "Dataset too large - only analyzed first {} of {} records",
                max_records_to_analyze,
                records.len()
            ),
        });
    }

    issues
}