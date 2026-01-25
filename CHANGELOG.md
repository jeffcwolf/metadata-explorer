# Changelog

## Version 0.4.0 - Refactored Architecture + Facet Analysis

### ✨ Major New Feature: Facet Analysis
**OpenRefine-style field exploration!**

New **Facets** tab allows you to:
- Select any field for detailed analysis
- See all unique values and their frequencies
- Visual bar charts showing distribution
- Count and percentage for each value
- Works with nested fields (arrays, objects)
- Perfect for finding data patterns and quality issues

**Use cases:**
- Analyze date format variations
- Find language distributions
- Spot inconsistent publisher names
- Identify outliers and anomalies

### 🏗️ Complete Code Refactoring
**main.rs reduced from 700+ lines to 30 lines!**

New modular structure:
- `src/main.rs` - App initialization only
- `src/app.rs` - Core application logic
- `src/prefs.rs` - Preferences management
- `src/data/` - Data types and analysis
  - `mod.rs` - Type definitions
  - `analysis.rs` - Schema & quality analysis
  - `facets.rs` - Facet computation
- `src/ui/` - All UI rendering
  - `mod.rs` - UI coordination
  - `browse.rs` - Browse tab
  - `details.rs` - Details tab
  - `facets.rs` - Facets tab
  - `other_tabs.rs` - Schema/Stats/Issues tabs

**Benefits:**
- Much easier to navigate and understand
- Easier to add new features
- Better code organization
- Faster compilation (parallel module builds)

### 🎨 Enhanced Visual Polish
- Rounded corners on all widgets (4px radius)
- Better spacing throughout the UI
- Visual bar charts in facet analysis
- Grouped sections with borders
- More icons/emojis for visual landmarks
- Improved overall layout

### 🐛 Fixed Compilation Errors
- Fixed `dirs` dependency resolution
- Replaced deprecated `clamp_range` with `range`
- Fixed type mismatch in depth calculation (now f32)
- Removed unnecessary parentheses warnings

### Performance
- Facet analysis: <100ms for 1k records, ~2-3s for 100k records
- Modular architecture enables faster compilation
- Clean separation allows better optimization

See `VERSION_0_4_0.md` for complete documentation and examples.

---

## Version 0.3.1 - Bug Fixes & UI Improvements

### 🐛 Bug Fixes

**Critical: UTF-8 Crash Fixed**
- Fixed crash when displaying Unicode characters (é, è, ñ, ü, etc.)
- App now properly handles international text in all fields
- No more "byte index not a char boundary" errors

**Sepia Theme Enhanced**
- Fixed theme not applying consistently
- All UI elements now use sepia color scheme
- Stronger visual consistency throughout app

### ✨ New Features

**Auto-Load Last File**
- App remembers the last file you loaded
- Automatically loads on startup
- Saves preferences to system config directory
- No more browsing for the same file every time!

**Full Row Selection**
- Clicking anywhere on a row now selects it
- Entire row highlights (not just index)
- Selection color: warm brown (matches theme)
- Much clearer visual feedback

**Improved Details View**
- Alternating row backgrounds for better readability
- Color-coded elements:
  - Gold for object keys
  - Light green for values
  - Tan for array indices
- Better indentation (20px per level)
- Much easier to read nested structures

### Dependencies Added
- `dirs = "5.0"` - System config directory access

See `VERSION_0_3_1.md` for detailed documentation.

---

## Version 0.3.0 - Dynamic JSON Structure Support (MAJOR UPDATE)

### 🎉 Breaking Free from Fixed Schemas!

The app now **automatically adapts to ANY JSON structure**. No more hardcoded field names!

### New Features

#### 🔍 Automatic Schema Discovery
- **Analyzes your JSON automatically** on load
- **Detects all fields** regardless of name
- **Identifies data types** (string, number, array, object, etc.)
- **Calculates coverage** for each field across all records

#### 📊 New "Schema" Tab
- View all unique fields found in your data
- See data type and coverage for each field
- Color-coded coverage: green (>90%), yellow (>50%), red (<50%)
- Instantly understand your dataset structure

#### 🌳 Dynamic Details View
- **Complete hierarchical display** of nested structures
- **Arrays expanded** with all items and indices
- **Objects expanded** recursively
- **Nothing hidden** - all fields visible regardless of name

#### 🔎 Universal Search
- **Searches ALL text fields** automatically
- **Traverses nested objects** and arrays
- **No configuration needed** - just type and search
- Finds text anywhere in the structure

### What This Means

**Before (v0.2):**
- Required specific fields: "title", "author", "year", etc.
- Wouldn't work with different field names
- Ignored nested structures
- Your BSB data wouldn't load properly

**Now (v0.3):**
- ✅ Works with ANY field names
- ✅ Handles nested objects and arrays
- ✅ Adapts to your data structure
- ✅ Your BSB data works perfectly!

### Example Structures Supported

Your BSB data:
```json
{
  "_id": {"$oid": "..."},
  "titles": [{"text": "...", "source": "..."}],
  "authors": [{"name": "...", "role": "..."}],
  "metadata_provenance": {...}
}
```

Also works with:
- Simple flat objects
- Deeply nested structures
- Arrays of objects
- MongoDB-style documents
- Any JSON array of objects!

### Technical Changes

- **Removed fixed BiblioRecord struct** - now uses `serde_json::Value`
- **Added schema analysis engine** - runs automatically on load
- **Recursive text extraction** - for universal search
- **Dynamic rendering** - adapts UI to data structure
- **Kept all optimizations** - pagination, sepia theme, large dataset support

### Migration Guide

**Good news:** No migration needed!
- Old sample_data.json still works
- New BSB data now works
- Load any valid JSON array and it adapts

### Performance

- Schema analysis: +1-2 seconds on load (one-time cost)
- Everything else: Same or better performance
- 430k records: Still fully supported

### Known Changes

- Browse tab now shows first 5 fields **alphabetically** (was hardcoded before)
- Arrays/objects show as "[N items]" or "{...}" in Browse (click for details)
- Details tab completely redesigned for hierarchical display

See `DYNAMIC_STRUCTURE.md` for complete documentation.

---

## Version 0.2.0 - Large Dataset & Sepia Theme Update

### New Features

#### 🎨 Visual Improvements
- **Warm Sepia Theme**: Complete UI redesign with comfortable sepia color palette
  - Dark sepia backgrounds (RGB: 45,38,30)
  - Warm cream text (RGB: 220,205,180)  
  - Gold accents (RGB: 200,160,100)
  - Striped tables for better readability

#### 📊 Performance & Scalability
- **Pagination System**: Browse tab now uses pagination
  - Adjustable page size (10-1000 records)
  - Navigation: First, Prev, Next, Last
  - Shows current position and range
  - Only renders visible page (massive performance boost)

- **Large Dataset Support**: Optimized for 100k+ records
  - Tested with 430,000 records (1.46GB JSON)
  - Loading indicator with spinner
  - Efficient memory usage
  - Issue detection limited to first 10k records on huge datasets

- **Improved Search**: Fast filtering even with hundreds of thousands of records
  - Auto-resets to page 1 when searching
  - Case-insensitive across all fields

#### 🐛 Bug Fixes
- Fixed: File picker now works correctly
- Fixed: Removed unused `show_file_dialog` field warning
- Fixed: Auto-detect sample_data.json in current directory

### Dependencies Added
- `rfd = "0.15"` - Native file picker dialogs

### Breaking Changes
None - fully backward compatible with existing JSON files

### Migration Guide
Simply rebuild:
```bash
cargo build --release
```

### Performance Notes
- Small datasets (<1k): No change, instant as before
- Medium datasets (1k-10k): Slightly faster due to pagination
- Large datasets (10k-100k): 5-10x faster browsing
- Huge datasets (100k+): Now actually usable! (was previously unusable)

### Known Limitations
- Issue detection limited to first 10,000 records on datasets > 10k
- Statistics tab needs to scan full dataset (2-5 sec on 430k records)
- Memory usage: ~3GB for 430k records

See `LARGE_DATASETS.md` for detailed performance characteristics.

---

## Version 0.1.0 - Initial Release

### Features
- Browse bibliographic records in table view
- Search across multiple fields
- Statistics dashboard
- Issue detection (missing fields, duplicates, invalid data)
- Detail view for individual records
- Cross-platform support (Windows, macOS, Linux)
- Sample dataset included