# Metadata Explorer

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.18663802.svg)](https://doi.org/10.5281/zenodo.18663802)

A fast, powerful desktop application for exploring and analyzing JSON datasets (including bibliographical records). Built in Rust with a comfortable sepia-themed interface.

## Overview

Bibliographical Metadata Explorer automatically adapts to any JSON structure, making it perfect for exploring bibliographic data, MongoDB exports, API responses, or any JSON array of objects. With support for datasets up to 430,000+ records, it's designed for both small datasets and large-scale data analysis.

## ✨ Key Features

### 🔍 **Universal JSON Support**
- **Automatic schema discovery** - Analyzes your data structure on load
- **Dynamic field detection** - Works with any field names, no configuration needed
- **Nested structure support** - Handles objects, arrays, and complex hierarchies
- **MongoDB-friendly** - Perfect for MongoDB exports and BSON-style documents

### 📊 **Facet Analysis** (OpenRefine-style)
- Explore any field in detail with frequency distributions
- Visual bar charts showing value distributions
- Count and percentage for each unique value
- Perfect for finding patterns, outliers, and data quality issues
- **Use cases**: Analyze date formats, language distributions, inconsistent values

### 🚀 **Performance & Scale**
- **Tested with 430,000 records** (1.46GB JSON files)
- Smart pagination system (10-1000 records per page)
- Fast search across all fields
- Efficient memory usage
- Loading time: ~2-5 seconds for huge datasets

### 🎨 **Comfortable Interface**
- **Warm sepia theme** - Easy on the eyes for long analysis sessions
- Clean, modern UI with rounded corners
- Striped tables for better readability
- Color-coded data types and coverage indicators
- Full row selection for easy navigation

### 📋 **Five Analysis Tabs**

1. **Browse** - Paginated table view of your data
2. **Details** - Hierarchical view of individual records with full nesting
3. **Facets** - OpenRefine-style field analysis with distributions
4. **Schema** - Automatic field detection with type and coverage info
5. **Statistics** - Dataset overview and quality metrics
6. **Issues** - Automatic detection of missing fields, duplicates, invalid data

### 🔎 **Smart Search**
- Searches ALL text fields automatically
- Traverses nested objects and arrays
- Case-insensitive
- Instant filtering even on large datasets

### 💾 **User Convenience**
- Auto-loads your last opened file on startup
- Remembers preferences (theme, page size, etc.)
- Native file picker dialogs

## 🎯 Use Cases

- **Bibliographic Data Analysis** - Explore library catalogs, publication records
- **MongoDB Data Exploration** - Analyze exported collections
- **API Response Investigation** - Understand complex API data structures
- **Data Quality Auditing** - Find inconsistencies, missing values, duplicates
- **Dataset Profiling** - Quick overview of field coverage and distributions

## 📦 Installation

### Prerequisites
- Rust toolchain (1.70+)
- Cargo package manager

### Build from Source
```bash
# Clone the repository
git clone <repository-url>
cd json-explorer

# Build release version
cargo build --release

# Binary will be in target/release/
```

### Run
```bash
cargo run --release
```

Or run the compiled binary directly:
```bash
./target/release/json-explorer  # Linux/macOS
target\release\json-explorer.exe  # Windows
```

## 🚀 Quick Start

1. **Launch the application**
2. **Load a JSON file** - Click "Load JSON" or place `sample_data.json` in the current directory
3. **Explore your data**:
   - **Browse** tab - Scroll through records with pagination
   - **Facets** tab - Select a field to see value distributions
   - **Schema** tab - View all detected fields and their types
   - **Search** - Type to filter across all text fields
   - **Details** - Click any record to see full nested structure

## 📊 Supported JSON Formats

### Simple Flat Objects
```json
[
  {"title": "Book 1", "author": "Smith", "year": 2020},
  {"title": "Book 2", "author": "Jones", "year": 2021}
]
```

### Nested Structures
```json
[
  {
    "_id": {"$oid": "..."},
    "titles": [{"text": "...", "source": "..."}],
    "authors": [{"name": "...", "role": "..."}],
    "metadata": {"created": "...", "updated": "..."}
  }
]
```

### MongoDB Exports
- ObjectId fields
- Nested documents
- Arrays of subdocuments
- Any BSON-compatible structure

## ⚡ Performance Characteristics

| Dataset Size | Load Time | Browse Performance | Notes |
|--------------|-----------|-------------------|-------|
| < 1,000 records | Instant | Instant | Full feature set |
| 1k - 10k | < 1 second | Fast | Full feature set |
| 10k - 100k | 1-3 seconds | Fast (paginated) | Issue detection limited to first 10k |
| 100k - 500k | 2-5 seconds | Fast (paginated) | Issue detection limited to first 10k |

**Memory usage**: ~3GB for 430,000 records

## 🛠️ Tech Stack

- **Language**: Rust
- **GUI Framework**: egui (immediate mode GUI)
- **JSON Processing**: serde_json
- **File Dialogs**: rfd
- **Config Storage**: dirs

## 📚 Version History

- **v0.4.0** - Facet analysis + complete code refactoring
- **v0.3.1** - UTF-8 crash fixes + auto-load last file
- **v0.3.0** - Dynamic JSON structure support (major update)
- **v0.2.0** - Large dataset support + sepia theme
- **v0.1.0** - Initial release

See [CHANGELOG.md](CHANGELOG.md) for detailed release notes.

## 🤝 Contributing

Contributions are welcome! The codebase is now modularly organized:
- `src/main.rs` - App initialization
- `src/app.rs` - Core application logic
- `src/data/` - Data types and analysis
- `src/ui/` - UI rendering components

## 📄 License

[Add your license here]

## 🙏 Acknowledgments

Inspired by OpenRefine's facet analysis approach.

---

**Built with ❤️ in Rust**