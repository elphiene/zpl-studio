// Offline ZPL preview rendering using zpl-forge

use image::DynamicImage;
use std::collections::HashMap;
use zpl_forge::{ZplEngine, Unit, Resolution};
use zpl_forge::forge::png::PngBackend;

/// Renders ZPL to a PNG image using zpl-forge (offline)
///
/// Parameters:
/// - zpl: The ZPL code to render
/// - dpi: DPI setting (203, 300, 600)
/// - width: Label width in inches
/// - height: Label height in inches
pub fn render_preview(zpl: &str, dpi: u16, width: f32, height: f32) -> Result<DynamicImage, String> {
    // Map DPI to zpl-forge Resolution enum
    let resolution = match dpi {
        203 => Resolution::Dpi203,
        300 => Resolution::Dpi300,
        600 => Resolution::Dpi600,
        _ => Resolution::Dpi203, // Default to 203 DPI
    };

    // Create ZPL engine
    let engine = ZplEngine::new(
        zpl,
        Unit::Inches(width),
        Unit::Inches(height),
        resolution
    ).map_err(|e| format!("Failed to parse ZPL: {:?}", e))?;

    // Render to PNG
    let png_backend = PngBackend::new();
    let png_bytes = engine
        .render(png_backend, &HashMap::new())
        .map_err(|e| format!("Failed to render ZPL: {:?}", e))?;

    // Decode PNG image
    let img = image::load_from_memory(&png_bytes)
        .map_err(|e| format!("Failed to decode preview image: {}", e))?;

    Ok(img)
}
