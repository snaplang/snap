use crate::scratch::ScratchProject;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

/// Default blank SVG for backdrop
const DEFAULT_BACKDROP_SVG: &str = r#"<svg version="1.1" width="2" height="2" viewBox="-1 -1 2 2" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
  <!-- Exported by Scratch - http://scratch.mit.edu/ -->
</svg>"#;

/// Default blank SVG for sprite costume (a simple shape)
const DEFAULT_COSTUME_SVG: &str = r##"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="96" height="100" viewBox="0 0 96 100">
  <g transform="translate(48,50)">
    <circle r="40" fill="#855CD6" stroke="#5B3D91" stroke-width="3"/>
    <circle cx="-15" cy="-10" r="8" fill="white"/>
    <circle cx="15" cy="-10" r="8" fill="white"/>
    <circle cx="-15" cy="-10" r="4" fill="black"/>
    <circle cx="15" cy="-10" r="4" fill="black"/>
    <ellipse cx="0" cy="15" rx="15" ry="10" fill="#F9A"/>
  </g>
</svg>"##;

pub fn package(project: &ScratchProject, output_path: &Path) -> Result<(), String> {
    let file =
        File::create(output_path).map_err(|e| format!("Failed to create output file: {}", e))?;

    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // Write project.json
    let project_json = serde_json::to_string_pretty(project)
        .map_err(|e| format!("Failed to serialize project: {}", e))?;

    zip.start_file("project.json", options)
        .map_err(|e| format!("Failed to start project.json: {}", e))?;
    zip.write_all(project_json.as_bytes())
        .map_err(|e| format!("Failed to write project.json: {}", e))?;

    // Write default assets
    // For now, we include default backdrop and costume SVGs
    // In the future, this should read actual files from the project

    // Default backdrop
    zip.start_file("cd21514d0531fdffb22204e0ec5ed84a.svg", options)
        .map_err(|e| format!("Failed to start backdrop asset: {}", e))?;
    zip.write_all(DEFAULT_BACKDROP_SVG.as_bytes())
        .map_err(|e| format!("Failed to write backdrop asset: {}", e))?;

    // Default costume
    zip.start_file("bcf454acf82e4504149f7ffe07081571.svg", options)
        .map_err(|e| format!("Failed to start costume asset: {}", e))?;
    zip.write_all(DEFAULT_COSTUME_SVG.as_bytes())
        .map_err(|e| format!("Failed to write costume asset: {}", e))?;

    zip.finish()
        .map_err(|e| format!("Failed to finish zip: {}", e))?;

    Ok(())
}
