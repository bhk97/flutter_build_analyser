# Flutter Build Analyser

A CLI tool to analyze Flutter projects, assets, dependencies, and build performance.

## Usage

```bash
cargo run -- <build-command> <project-path> [output-type]
```

### Arguments
- `<build-command>`: The Flutter build type (e.g., `apk`, `appbundle`, `ios`, `ipa`, `web`, `linux`, `macos`, `windows`).
- `<project-path>`: Path to the root of the Flutter project.
- `[output-type]`: Optional. Set to `json` for JSON formatted output.

### Examples

**Standard analysis:**
```bash
cargo run -- apk /path/to/flutter_project
```

**JSON output:**
```bash
cargo run -- apk /path/to/flutter_project json
```

## Features
- **Asset Analysis**: Calculates asset sizes and identifies unused assets.
- **Dependency Analysis**: Summarizes the project dependency graph.
- **Build Timing**: Measures duration of various build phases.
- **APK Breakdown**: Analyzes the size and content of the generated APK.
