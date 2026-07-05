//! Pure-Rust conversion of Office documents (DOCX, PPTX, XLSX) to PDF, PNG, or JPEG.
//!
//! # Quick start — PDF
//!
//! ```no_run
//! # let input_bytes = std::fs::read("input.docx").unwrap();
//! let result = office_print::convert_bytes(
//!     &input_bytes,
//!     office_print::config::Format::Docx,
//!     &office_print::config::ConvertOptions::default(),
//! ).unwrap();
//! std::fs::write("report.pdf", result.as_pdf_bytes().unwrap()).unwrap();
//! ```
//!
//! # PNG output
//!
//! ```no_run
//! # let docx_bytes = std::fs::read("input.docx").unwrap();
//! use office_print::config::{ConvertOptions, Format, OutputFormat};
//!
//! let options = ConvertOptions {
//!     output_format: OutputFormat::Png,
//!     ..Default::default()
//! };
//! let result = office_print::convert_bytes(&docx_bytes, Format::Docx, &options).unwrap();
//! for (i, page) in result.output.as_raster_pages().unwrap().iter().enumerate() {
//!     std::fs::write(format!("page-{}.png", i + 1), page).unwrap();
//! }
//! ```
//!
//! # JPEG output
//!
//! ```no_run
//! # let docx_bytes = std::fs::read("input.docx").unwrap();
//! use office_print::config::{ConvertOptions, Format, OutputFormat};
//!
//! let options = ConvertOptions {
//!     output_format: OutputFormat::Jpeg,
//!     jpeg_quality: 85,
//!     ..Default::default()
//! };
//! let result = office_print::convert_bytes(&docx_bytes, Format::Docx, &options).unwrap();
//! for (i, page) in result.output.as_raster_pages().unwrap().iter().enumerate() {
//!     std::fs::write(format!("page-{}.jpg", i + 1), page).unwrap();
//! }
//! ```

pub mod config;
pub mod defaults;
pub mod error;
pub mod ir;
pub mod parser;
#[cfg(feature = "pdf-ops")]
pub mod pdf_ops;
pub mod render;
#[cfg(feature = "wasm")]
pub mod wasm;

use config::{ConvertOptions, Format};
use error::{ConvertError, ConvertResult};
#[path = "lib_pipeline.rs"]
mod pipeline;
#[cfg(test)]
#[path = "lib_test_support.rs"]
mod test_support;

#[cfg(test)]
fn is_ole2(data: &[u8]) -> bool {
    pipeline::is_ole2(data)
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
fn should_resolve_font_context(doc: &ir::Document, options: &ConvertOptions) -> bool {
    pipeline::should_resolve_font_context(doc, options, false)
}

/// Convert a file at the given path to PDF bytes with warnings.
///
/// Detects the format from the file extension (`.docx`, `.pptx`, `.xlsx`).
///
/// This function is not available on `wasm32` targets because it reads from the
/// filesystem. Use [`convert_bytes`] for in-memory conversion on WASM.
///
/// # Errors
///
/// Returns [`ConvertError::UnsupportedFormat`] if the extension is unrecognized,
/// [`ConvertError::Io`] if the file cannot be read, or other variants for
/// parse/render failures.
#[cfg(not(target_arch = "wasm32"))]
pub fn convert(path: impl AsRef<std::path::Path>) -> Result<ConvertResult, ConvertError> {
    pipeline::convert(path)
}

/// Convert a file at the given path to PDF bytes with options.
///
/// See [`ConvertOptions`] for available settings (paper size, sheet filter, etc.).
///
/// This function is not available on `wasm32` targets because it reads from the
/// filesystem. Use [`convert_bytes`] for in-memory conversion on WASM.
///
/// # Errors
///
/// Returns [`ConvertError`] on unsupported format, I/O, parse, or render failure.
#[cfg(not(target_arch = "wasm32"))]
pub fn convert_with_options(
    path: impl AsRef<std::path::Path>,
    options: &ConvertOptions,
) -> Result<ConvertResult, ConvertError> {
    pipeline::convert_with_options(path, options)
}

/// Convert raw bytes of a known format to PDF bytes with warnings.
///
/// Use this when you already have the file contents in memory and know the
/// [`Format`].
///
/// When `options.streaming` is `true` and the format is XLSX, the conversion
/// processes rows in chunks to bound peak memory during Typst compilation.
/// This requires the `pdf-ops` feature for PDF merging.
///
/// # Errors
///
/// Returns [`ConvertError`] on parse or render failure.
pub fn convert_bytes(
    data: &[u8],
    format: Format,
    options: &ConvertOptions,
) -> Result<ConvertResult, ConvertError> {
    pipeline::convert_bytes(data, format, options)
}

/// Render an IR Document to PDF bytes.
///
///// Render an IR [`Document`](ir::Document) directly to PDF bytes.
///
/// Takes a fully constructed [`ir::Document`] and runs it through
/// the Typst codegen → PDF compilation pipeline.
///
/// # Errors
///
/// Returns [`ConvertError::Render`] if Typst compilation or PDF export fails.
pub fn render_document(doc: &ir::Document) -> Result<Vec<u8>, ConvertError> {
    pipeline::render_document(doc)
}

#[cfg(test)]
#[path = "lib_pipeline_tests.rs"]
mod pipeline_tests;

#[cfg(test)]
#[path = "lib_render_tests.rs"]
mod render_tests;

#[cfg(test)]
#[path = "lib_conversion_tests.rs"]
mod conversion_tests;

#[cfg(test)]
#[path = "lib_robustness_tests.rs"]
mod robustness_tests;

#[cfg(all(test, feature = "typescript"))]
#[path = "lib_ts_integration_tests.rs"]
mod ts_integration_tests;

#[cfg(all(test, feature = "pdf-ops"))]
#[path = "lib_streaming_tests.rs"]
mod streaming_tests;
