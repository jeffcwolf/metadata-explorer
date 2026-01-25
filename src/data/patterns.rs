use serde_json::Value;
use std::collections::HashMap;
use super::{BiblioRecord, FacetAnalysis, FacetValue};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternType {
    // Structural patterns (generic)
    Empty,
    VeryLong,
    VeryShort,
    PureNumeric,
    FourDigitYear,
    IsoDate,
    EmailLike,
    UrlLike,
    AllCaps,
    NumericWithPunctuation,
    
    // Bibliographic domain patterns
    FuzzyDate,
    DateRange,
    CenturyNotation,
    IsoLanguageCode,
    BracketedContent,
    
    // Fallback
    MixedAlphanumeric,
    SpecialCharacterHeavy,
    Other,
}

impl PatternType {
    pub fn name(&self) -> &str {
        match self {
            PatternType::Empty => "Empty/Whitespace",
            PatternType::VeryLong => "Very Long (>100 chars)",
            PatternType::VeryShort => "Very Short (1-2 chars)",
            PatternType::PureNumeric => "Purely Numeric",
            PatternType::FourDigitYear => "4-Digit Years (1000-2999)",
            PatternType::IsoDate => "ISO Date Format",
            PatternType::EmailLike => "Email-like",
            PatternType::UrlLike => "URL-like",
            PatternType::AllCaps => "All Uppercase",
            PatternType::NumericWithPunctuation => "Numbers with Punctuation",
            PatternType::FuzzyDate => "Fuzzy Dates (circa, ca., ~)",
            PatternType::DateRange => "Date Ranges",
            PatternType::CenturyNotation => "Century Notation",
            PatternType::IsoLanguageCode => "ISO Language Code (3-letter)",
            PatternType::BracketedContent => "Bracketed Content",
            PatternType::MixedAlphanumeric => "Mixed Alphanumeric",
            PatternType::SpecialCharacterHeavy => "Special Character Heavy",
            PatternType::Other => "Other/Unclassified",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            PatternType::Empty => "Empty strings or whitespace only",
            PatternType::VeryLong => "Strings longer than 100 characters",
            PatternType::VeryShort => "1-2 character strings (codes, initials)",
            PatternType::PureNumeric => "Only digits, no other characters",
            PatternType::FourDigitYear => "Four-digit numbers in year range (1000-2999)",
            PatternType::IsoDate => "ISO 8601 date format (YYYY-MM-DD or with time)",
            PatternType::EmailLike => "Contains @ symbol",
            PatternType::UrlLike => "Starts with http:// or https://",
            PatternType::AllCaps => "All uppercase letters (acronyms, codes)",
            PatternType::NumericWithPunctuation => "Numbers with commas, periods, or hyphens",
            PatternType::FuzzyDate => "Approximate dates: 'circa 1800', 'ca. 1850', '~1900'",
            PatternType::DateRange => "Date ranges: '1800-1850', '1999/2000'",
            PatternType::CenturyNotation => "Century references: '18th century', '19. Jahrhundert'",
            PatternType::IsoLanguageCode => "3-letter language codes (eng, ger, fre)",
            PatternType::BracketedContent => "Text within brackets: '[n.d.]', '[London]'",
            PatternType::MixedAlphanumeric => "Mix of letters and numbers",
            PatternType::SpecialCharacterHeavy => ">25% special characters",
            PatternType::Other => "Does not match any specific pattern",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatternGroup {
    pub pattern_type: PatternType,
    pub count: usize,
    pub percentage: f32,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PatternAnalysis {
    pub field_name: String,
    pub total_values: usize,
    pub pattern_groups: Vec<PatternGroup>,
}

pub fn analyze_patterns(facets: &FacetAnalysis) -> PatternAnalysis {
    let mut pattern_map: HashMap<PatternType, (usize, Vec<String>)> = HashMap::new();
    
    for facet_value in &facets.values {
        let pattern = classify_value(&facet_value.value);
        
        let entry = pattern_map.entry(pattern).or_insert((0, Vec::new()));
        entry.0 += facet_value.count;
        
        // Collect examples (up to 5 per pattern)
        if entry.1.len() < 5 {
            entry.1.push(facet_value.value.clone());
        }
    }
    
    let mut pattern_groups: Vec<PatternGroup> = pattern_map
        .into_iter()
        .map(|(pattern_type, (count, examples))| {
            let percentage = if facets.total_values > 0 {
                (count as f32 / facets.total_values as f32) * 100.0
            } else {
                0.0
            };
            
            PatternGroup {
                pattern_type,
                count,
                percentage,
                examples,
            }
        })
        .collect();
    
    // Sort by count descending
    pattern_groups.sort_by(|a, b| b.count.cmp(&a.count));
    
    PatternAnalysis {
        field_name: facets.field_name.clone(),
        total_values: facets.total_values,
        pattern_groups,
    }
}

fn classify_value(value: &str) -> PatternType {
    let trimmed = value.trim();
    
    // Early returns for obvious cases
    if trimmed.is_empty() {
        return PatternType::Empty;
    }
    
    if trimmed.len() > 100 {
        return PatternType::VeryLong;
    }
    
    if trimmed.len() <= 2 {
        return PatternType::VeryShort;
    }
    
    // URL check (early because common)
    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        return PatternType::UrlLike;
    }
    
    // Email check
    if trimmed.contains('@') {
        return PatternType::EmailLike;
    }
    
    // Bibliographic patterns - fuzzy dates
    if is_fuzzy_date(trimmed) {
        return PatternType::FuzzyDate;
    }
    
    // Bibliographic patterns - date ranges
    if is_date_range(trimmed) {
        return PatternType::DateRange;
    }
    
    // Bibliographic patterns - century notation
    if is_century_notation(trimmed) {
        return PatternType::CenturyNotation;
    }
    
    // Bibliographic patterns - bracketed content
    if is_bracketed_content(trimmed) {
        return PatternType::BracketedContent;
    }
    
    // ISO date format (YYYY-MM-DD or YYYY-MM-DD HH:MM:SS)
    if is_iso_date(trimmed) {
        return PatternType::IsoDate;
    }
    
    // 4-digit year check
    if is_four_digit_year(trimmed) {
        return PatternType::FourDigitYear;
    }
    
    // ISO language code (3 letters, all lowercase or uppercase)
    if is_iso_language_code(trimmed) {
        return PatternType::IsoLanguageCode;
    }
    
    // Numeric checks
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return PatternType::PureNumeric;
    }
    
    if is_numeric_with_punctuation(trimmed) {
        return PatternType::NumericWithPunctuation;
    }
    
    // All caps check
    if trimmed.len() >= 3 && trimmed.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        return PatternType::AllCaps;
    }
    
    // Special character heavy
    let special_count = trimmed.chars().filter(|c| !c.is_alphanumeric() && !c.is_whitespace()).count();
    if special_count as f32 / trimmed.len() as f32 > 0.25 {
        return PatternType::SpecialCharacterHeavy;
    }
    
    // Mixed alphanumeric
    let has_alpha = trimmed.chars().any(|c| c.is_alphabetic());
    let has_digit = trimmed.chars().any(|c| c.is_ascii_digit());
    if has_alpha && has_digit {
        return PatternType::MixedAlphanumeric;
    }
    
    // Default
    PatternType::Other
}

fn is_four_digit_year(value: &str) -> bool {
    if value.len() != 4 {
        return false;
    }
    
    if let Ok(year) = value.parse::<i32>() {
        year >= 1000 && year <= 2999
    } else {
        false
    }
}

fn is_iso_date(value: &str) -> bool {
    // Simple check for YYYY-MM-DD or YYYY-MM-DD HH:MM:SS patterns
    if value.len() < 10 {
        return false;
    }
    
    // Check for basic pattern: YYYY-MM-DD
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() >= 3 {
        if parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit()) &&
           parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit()) &&
           parts[2].starts_with(|c: char| c.is_ascii_digit()) {
            return true;
        }
    }
    
    false
}

fn is_fuzzy_date(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("circa") || 
    lower.contains("ca.") || 
    lower.contains("ca ") ||
    lower.starts_with("~") ||
    lower.contains("c.") && (lower.contains("18") || lower.contains("19") || lower.contains("20"))
}

fn is_date_range(value: &str) -> bool {
    // Look for patterns like "1800-1850" or "1999/2000"
    let has_year_separator = value.contains('-') || value.contains('/');
    
    if !has_year_separator {
        return false;
    }
    
    // Count digits - should have at least 8 for two 4-digit years
    let digit_count = value.chars().filter(|c| c.is_ascii_digit()).count();
    if digit_count < 8 {
        return false;
    }
    
    // Simple heuristic: if it has a separator and enough digits, likely a date range
    let separators = value.matches('-').count() + value.matches('/').count();
    separators == 1 && digit_count >= 8
}

fn is_century_notation(value: &str) -> bool {
    let lower = value.to_lowercase();
    
    // Check for patterns like "18th century", "19. Jahrhundert"
    (lower.contains("century") || lower.contains("jahrhundert")) &&
    (lower.contains("th") || lower.contains("st") || lower.contains("nd") || lower.contains("rd") || lower.contains('.'))
}

fn is_iso_language_code(value: &str) -> bool {
    // ISO 639-2/T codes are exactly 3 letters
    value.len() == 3 && value.chars().all(|c| c.is_alphabetic())
}

fn is_bracketed_content(value: &str) -> bool {
    // Check for content within brackets
    (value.starts_with('[') && value.ends_with(']')) ||
    (value.starts_with('(') && value.ends_with(')'))
}

fn is_numeric_with_punctuation(value: &str) -> bool {
    let has_digits = value.chars().any(|c| c.is_ascii_digit());
    let has_punctuation = value.contains(',') || value.contains('.') || value.contains('-');
    let mostly_numeric_or_punct = value.chars()
        .filter(|c| !c.is_whitespace())
        .all(|c| c.is_ascii_digit() || c == ',' || c == '.' || c == '-');
    
    has_digits && has_punctuation && mostly_numeric_or_punct
}