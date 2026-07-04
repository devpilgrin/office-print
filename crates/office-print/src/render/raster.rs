use image::ImageEncoder;
use typst::layout::{Page, PagedDocument};

use crate::error::ConvertError;

/// Render a single page to RGBA pixel data via typst-render.
/// Returns (rgba_bytes, width_px, height_px).
fn render_page_rgba(page: &Page, pixel_per_pt: f32) -> (Vec<u8>, u32, u32) {
    let pixmap = typst_render::render(page, pixel_per_pt);
    let w = pixmap.width();
    let h = pixmap.height();
    let rgba = unpremultiply(pixmap.data(), w, h);
    (rgba, w, h)
}

/// Un-premultiply RGBA pixel data.
fn unpremultiply(data: &[u8], _width: u32, _height: u32) -> Vec<u8> {
    let mut out = data.to_vec();
    for chunk in out.chunks_exact_mut(4) {
        let a = chunk[3];
        if a > 0 && a < 255 {
            chunk[0] = ((chunk[0] as u16) * 255 / (a as u16)) as u8;
            chunk[1] = ((chunk[1] as u16) * 255 / (a as u16)) as u8;
            chunk[2] = ((chunk[2] as u16) * 255 / (a as u16)) as u8;
        } else if a == 0 {
            chunk[0] = 0;
            chunk[1] = 0;
            chunk[2] = 0;
        }
    }
    out
}

/// Encode RGBA pixel data as PNG bytes.
fn encode_png(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>, ConvertError> {
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder
        .write_image(rgba, width, height, image::ExtendedColorType::Rgba8)
        .map_err(|e| ConvertError::Render(format!("PNG encoding failed: {e}")))?;
    Ok(buf)
}

/// Encode RGBA pixel data as JPEG bytes with specified quality.
fn encode_jpeg(
    rgba: &[u8],
    width: u32,
    height: u32,
    quality: u8,
) -> Result<Vec<u8>, ConvertError> {
    let rgb: Vec<u8> = rgba
        .chunks_exact(4)
        .flat_map(|c| [c[0], c[1], c[2]])
        .collect();
    let mut buf = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    encoder
        .write_image(&rgb, width, height, image::ExtendedColorType::Rgb8)
        .map_err(|e| ConvertError::Render(format!("JPEG encoding failed: {e}")))?;
    Ok(buf)
}

/// Render all pages of a PagedDocument to PNG bytes (one `Vec<u8>` per page).
pub fn compile_to_png(document: &PagedDocument) -> Result<Vec<Vec<u8>>, ConvertError> {
    let pixel_per_pt: f32 = 2.0; // 2x = 144 DPI equivalent
    document
        .pages
        .iter()
        .map(|page| {
            let (rgba, w, h) = render_page_rgba(page, pixel_per_pt);
            encode_png(&rgba, w, h)
        })
        .collect()
}

/// Render all pages of a PagedDocument to JPEG bytes (one `Vec<u8>` per page).
pub fn compile_to_jpeg(
    document: &PagedDocument,
    quality: u8,
) -> Result<Vec<Vec<u8>>, ConvertError> {
    let pixel_per_pt: f32 = 2.0;
    document
        .pages
        .iter()
        .map(|page| {
            let (rgba, w, h) = render_page_rgba(page, pixel_per_pt);
            encode_jpeg(&rgba, w, h, quality)
        })
        .collect()
}
