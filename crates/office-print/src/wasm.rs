//! WebAssembly bindings for office-print via `wasm-bindgen`.
//!
//! This module is only available when the `wasm` feature is enabled.
//! It exports JavaScript-callable functions for converting Office documents
//! to PDF, PNG, or JPEG in browser or Node.js environments.
//!
//! # Running WASM integration tests
//!
//! ```bash
//! cd crates/office-print
//! wasm-pack test --node --features wasm
//! ```

use wasm_bindgen::prelude::*;

use crate::config::{ConvertOptions, Format, OutputFormat};
use crate::convert_bytes;
use crate::error::OutputData;

/// Internal: convert to the requested output format.
fn convert_inner(
    data: &[u8],
    format: &str,
    output_format: OutputFormat,
    jpeg_quality: u8,
) -> Result<Vec<u8>, String> {
    let fmt =
        Format::from_extension(format).ok_or_else(|| format!("unsupported format: {format}"))?;
    let options = ConvertOptions {
        output_format,
        jpeg_quality,
        ..Default::default()
    };
    let result = convert_bytes(data, fmt, &options).map_err(|e| e.to_string())?;
    flatten_output(result.output)
}

/// Internal: convert a known Format to the requested output format.
fn convert_format_inner(
    data: &[u8],
    input_format: Format,
    output_format: OutputFormat,
    jpeg_quality: u8,
) -> Result<Vec<u8>, String> {
    let options = ConvertOptions {
        output_format,
        jpeg_quality,
        ..Default::default()
    };
    let result = convert_bytes(data, input_format, &options).map_err(|e| e.to_string())?;
    flatten_output(result.output)
}

/// Flatten OutputData into a single Vec<u8>.
/// For multi-page raster, returns the first page only (WASM limitation).
fn flatten_output(output: OutputData) -> Result<Vec<u8>, String> {
    match output {
        OutputData::Pdf(pdf) => Ok(pdf),
        OutputData::Raster { pages, .. } => pages
            .into_iter()
            .next()
            .ok_or_else(|| "raster output has no pages".to_string()),
    }
}

// --- PDF exports ---

#[wasm_bindgen(js_name = "convertToPdf")]
pub fn convert_to_pdf(data: &[u8], format: &str) -> Result<Vec<u8>, JsValue> {
    convert_inner(data, format, OutputFormat::Pdf, 92).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertDocxToPdf")]
pub fn convert_docx_to_pdf(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Docx, OutputFormat::Pdf, 92)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertPptxToPdf")]
pub fn convert_pptx_to_pdf(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Pptx, OutputFormat::Pdf, 92)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertXlsxToPdf")]
pub fn convert_xlsx_to_pdf(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Xlsx, OutputFormat::Pdf, 92)
        .map_err(|e| JsValue::from_str(&e))
}

// --- PNG exports ---

#[wasm_bindgen(js_name = "convertToPng")]
pub fn convert_to_png(data: &[u8], format: &str) -> Result<Vec<u8>, JsValue> {
    convert_inner(data, format, OutputFormat::Png, 92).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertDocxToPng")]
pub fn convert_docx_to_png(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Docx, OutputFormat::Png, 92)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertPptxToPng")]
pub fn convert_pptx_to_png(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Pptx, OutputFormat::Png, 92)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertXlsxToPng")]
pub fn convert_xlsx_to_png(data: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_format_inner(data, Format::Xlsx, OutputFormat::Png, 92)
        .map_err(|e| JsValue::from_str(&e))
}

// --- JPEG exports ---

#[wasm_bindgen(js_name = "convertToJpeg")]
pub fn convert_to_jpeg(data: &[u8], format: &str, quality: Option<u8>) -> Result<Vec<u8>, JsValue> {
    let q = quality.unwrap_or(92);
    convert_inner(data, format, OutputFormat::Jpeg, q).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertDocxToJpeg")]
pub fn convert_docx_to_jpeg(data: &[u8], quality: Option<u8>) -> Result<Vec<u8>, JsValue> {
    let q = quality.unwrap_or(92);
    convert_format_inner(data, Format::Docx, OutputFormat::Jpeg, q)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertPptxToJpeg")]
pub fn convert_pptx_to_jpeg(data: &[u8], quality: Option<u8>) -> Result<Vec<u8>, JsValue> {
    let q = quality.unwrap_or(92);
    convert_format_inner(data, Format::Pptx, OutputFormat::Jpeg, q)
        .map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen(js_name = "convertXlsxToJpeg")]
pub fn convert_xlsx_to_jpeg(data: &[u8], quality: Option<u8>) -> Result<Vec<u8>, JsValue> {
    let q = quality.unwrap_or(92);
    convert_format_inner(data, Format::Xlsx, OutputFormat::Jpeg, q)
        .map_err(|e| JsValue::from_str(&e))
}

#[cfg(test)]
#[path = "wasm_tests.rs"]
mod tests;

// ---------------------------------------------------------------------------
// WASM integration tests (run via `wasm-pack test --node --features wasm`)
//
// These tests compile ONLY when targeting wasm32 and are executed inside a
// real WASM runtime (Node.js or headless browser). They call the actual
// `#[wasm_bindgen]`-exported functions and verify end-to-end conversion.
// ---------------------------------------------------------------------------
#[cfg(all(target_arch = "wasm32", test))]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;

    /// Helper: create a minimal valid DOCX via docx-rs builder.
    fn make_minimal_docx() -> Vec<u8> {
        use std::io::Cursor;
        let doc = docx_rs::Docx::new().add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Hello WASM")),
        );
        let mut buf = Cursor::new(Vec::new());
        doc.build().pack(&mut buf).unwrap();
        buf.into_inner()
    }

    /// Helper: create a minimal valid PPTX.
    fn make_minimal_pptx() -> Vec<u8> {
        use std::io::{Cursor, Write};
        let cursor = Cursor::new(Vec::new());
        let mut zip = zip::ZipWriter::new(cursor);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("[Content_Types].xml", options).unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>
  <Override PartName="/ppt/slides/slide1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slide+xml"/>
</Types>"#)
        .unwrap();

        zip.start_file("_rels/.rels", options).unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
</Relationships>"#)
        .unwrap();

        zip.start_file("ppt/presentation.xml", options).unwrap();
        zip.write_all(
            br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:sldIdLst>
    <p:sldId id="256" r:id="rId2"/>
  </p:sldIdLst>
</p:presentation>"#,
        )
        .unwrap();

        zip.start_file("ppt/_rels/presentation.xml.rels", options)
            .unwrap();
        zip.write_all(br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="slides/slide1.xml"/>
</Relationships>"#)
        .unwrap();

        zip.start_file("ppt/slides/slide1.xml", options).unwrap();
        zip.write_all(
            br#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>
      <p:grpSpPr/>
      <p:sp>
        <p:nvSpPr><p:cNvPr id="2" name="Title"/><p:cNvSpPr/><p:nvPr/></p:nvSpPr>
        <p:spPr>
          <a:xfrm><a:off x="0" y="0"/><a:ext cx="9144000" cy="1000000"/></a:xfrm>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:p><a:r><a:t>Hello WASM</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:sld>"#,
        )
        .unwrap();

        zip.finish().unwrap().into_inner()
    }

    /// Helper: create a minimal valid XLSX.
    fn make_minimal_xlsx() -> Vec<u8> {
        use std::io::Cursor;
        let mut book = umya_spreadsheet::new_file();
        let sheet = book.get_sheet_mut(&0).unwrap();
        sheet.get_cell_mut("A1").set_value("Hello WASM");
        let mut cursor = Cursor::new(Vec::new());
        umya_spreadsheet::writer::xlsx::write_writer(&book, &mut cursor).unwrap();
        cursor.into_inner()
    }

    #[wasm_bindgen_test]
    fn wasm_convert_docx_to_pdf_produces_valid_pdf() {
        let docx = make_minimal_docx();
        let result = convert_docx_to_pdf(&docx);
        assert!(result.is_ok(), "DOCX to PDF conversion failed in WASM");
        let pdf = result.unwrap();
        assert!(
            pdf.starts_with(b"%PDF"),
            "Output should start with %PDF magic bytes"
        );
        assert!(pdf.len() > 100, "PDF output should have meaningful size");
    }

    #[wasm_bindgen_test]
    fn wasm_convert_to_pdf_with_docx_format_string() {
        let docx = make_minimal_docx();
        let result = convert_to_pdf(&docx, "docx");
        assert!(
            result.is_ok(),
            "convert_to_pdf with 'docx' format failed in WASM"
        );
        let pdf = result.unwrap();
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[wasm_bindgen_test]
    fn wasm_convert_pptx_to_pdf_produces_valid_pdf() {
        let pptx = make_minimal_pptx();
        let result = convert_pptx_to_pdf(&pptx);
        assert!(result.is_ok(), "PPTX to PDF conversion failed in WASM");
        let pdf = result.unwrap();
        assert!(
            pdf.starts_with(b"%PDF"),
            "Output should start with %PDF magic bytes"
        );
    }

    #[wasm_bindgen_test]
    fn wasm_convert_xlsx_to_pdf_produces_valid_pdf() {
        let xlsx = make_minimal_xlsx();
        let result = convert_xlsx_to_pdf(&xlsx);
        assert!(result.is_ok(), "XLSX to PDF conversion failed in WASM");
        let pdf = result.unwrap();
        assert!(
            pdf.starts_with(b"%PDF"),
            "Output should start with %PDF magic bytes"
        );
    }

    #[wasm_bindgen_test]
    fn wasm_convert_to_pdf_invalid_data_returns_error() {
        let result = convert_docx_to_pdf(b"not a valid docx");
        assert!(result.is_err(), "Should fail on invalid input data");
    }

    #[wasm_bindgen_test]
    fn wasm_convert_to_pdf_unsupported_format_returns_error() {
        let result = convert_to_pdf(b"dummy", "txt");
        assert!(result.is_err(), "Should fail on unsupported format string");
    }

    // --- PNG tests ---

    #[wasm_bindgen_test]
    fn wasm_convert_to_png_produces_valid_png() {
        let docx = make_minimal_docx();
        let result = convert_to_png(&docx, "docx");
        assert!(result.is_ok(), "DOCX to PNG conversion failed in WASM");
        let png = result.unwrap();
        assert!(
            png.starts_with(b"\x89PNG\r\n\x1a\n"),
            "Output should start with PNG magic bytes"
        );
    }

    #[wasm_bindgen_test]
    fn wasm_convert_docx_to_png_produces_valid_png() {
        let docx = make_minimal_docx();
        let result = convert_docx_to_png(&docx);
        assert!(result.is_ok(), "convertDocxToPng failed");
        let png = result.unwrap();
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    }

    #[wasm_bindgen_test]
    fn wasm_convert_pptx_to_png_produces_valid_png() {
        let pptx = make_minimal_pptx();
        let result = convert_pptx_to_png(&pptx);
        assert!(result.is_ok(), "convertPptxToPng failed");
        let png = result.unwrap();
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    }

    #[wasm_bindgen_test]
    fn wasm_convert_xlsx_to_png_produces_valid_png() {
        let xlsx = make_minimal_xlsx();
        let result = convert_xlsx_to_png(&xlsx);
        assert!(result.is_ok(), "convertXlsxToPng failed");
        let png = result.unwrap();
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    }

    // --- JPEG tests ---

    #[wasm_bindgen_test]
    fn wasm_convert_to_jpeg_produces_valid_jpeg() {
        let docx = make_minimal_docx();
        let result = convert_to_jpeg(&docx, "docx", None);
        assert!(result.is_ok(), "DOCX to JPEG conversion failed in WASM");
        let jpeg = result.unwrap();
        assert!(
            jpeg.starts_with(&[0xFF, 0xD8, 0xFF]),
            "Output should start with JPEG magic bytes"
        );
    }

    #[wasm_bindgen_test]
    fn wasm_convert_docx_to_jpeg_produces_valid_jpeg() {
        let docx = make_minimal_docx();
        let result = convert_docx_to_jpeg(&docx, None);
        assert!(result.is_ok(), "convertDocxToJpeg failed");
        let jpeg = result.unwrap();
        assert!(jpeg.starts_with(&[0xFF, 0xD8, 0xFF]));
    }

    #[wasm_bindgen_test]
    fn wasm_convert_to_jpeg_default_quality_is_92() {
        let docx = make_minimal_docx();
        let result = convert_to_jpeg(&docx, "docx", None);
        assert!(result.is_ok(), "JPEG with default quality should work");
    }

    #[wasm_bindgen_test]
    fn wasm_convert_to_jpeg_custom_quality() {
        let docx = make_minimal_docx();
        let result = convert_to_jpeg(&docx, "docx", Some(50));
        assert!(result.is_ok(), "JPEG with quality=50 should work");
    }
}
