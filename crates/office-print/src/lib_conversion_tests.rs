#![cfg(not(target_arch = "wasm32"))] // native-only unit tests (filesystem, system fonts)
use super::test_support::{build_test_docx, build_test_pptx, build_test_xlsx};
use super::*;

#[test]
fn test_e2e_docx_to_pdf() {
    let docx_bytes = build_test_docx();
    let result = convert_bytes(&docx_bytes, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        !result.as_pdf_bytes().unwrap().is_empty(),
        "DOCX→PDF should produce non-empty output"
    );
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "Output should be valid PDF"
    );
    assert!(
        result.warnings.is_empty(),
        "Normal DOCX should produce no warnings"
    );
}

#[test]
fn test_e2e_xlsx_to_pdf() {
    let xlsx_bytes = build_test_xlsx();
    let result = convert_bytes(&xlsx_bytes, Format::Xlsx, &ConvertOptions::default()).unwrap();
    assert!(
        !result.as_pdf_bytes().unwrap().is_empty(),
        "XLSX→PDF should produce non-empty output"
    );
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "Output should be valid PDF"
    );
}

#[test]
fn test_e2e_pptx_to_pdf() {
    let pptx_bytes = build_test_pptx();
    let result = convert_bytes(&pptx_bytes, Format::Pptx, &ConvertOptions::default()).unwrap();
    assert!(
        !result.as_pdf_bytes().unwrap().is_empty(),
        "PPTX→PDF should produce non-empty output"
    );
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "Output should be valid PDF"
    );
}

#[test]
fn test_e2e_docx_with_table_to_pdf() {
    use std::io::Cursor;

    let table = docx_rs::Table::new(vec![docx_rs::TableRow::new(vec![
        docx_rs::TableCell::new().add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Cell A")),
        ),
        docx_rs::TableCell::new().add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Cell B")),
        ),
    ])]);
    let docx = docx_rs::Docx::new()
        .add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Table below:")),
        )
        .add_table(table);
    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).unwrap();
    let data = cursor.into_inner();

    let result = convert_bytes(&data, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(!result.as_pdf_bytes().unwrap().is_empty());
    assert!(result.as_pdf_bytes().unwrap().starts_with(b"%PDF"));
}

#[test]
fn test_e2e_convert_with_options_from_temp_file() {
    let docx_bytes = build_test_docx();
    let dir = std::env::temp_dir();
    let input = dir.join("office_print_test_input.docx");
    let output = dir.join("office_print_test_output.pdf");
    std::fs::write(&input, &docx_bytes).unwrap();

    let result = convert(&input).unwrap();
    assert!(!result.as_pdf_bytes().unwrap().is_empty());
    assert!(result.as_pdf_bytes().unwrap().starts_with(b"%PDF"));

    let result2 = convert_with_options(&input, &ConvertOptions::default()).unwrap();
    assert!(!result2.as_pdf_bytes().unwrap().is_empty());
    assert!(result2.as_pdf_bytes().unwrap().starts_with(b"%PDF"));

    std::fs::write(&output, result.as_pdf_bytes().unwrap()).unwrap();
    assert!(output.exists());
    let written = std::fs::read(&output).unwrap();
    assert!(written.starts_with(b"%PDF"));

    let _ = std::fs::remove_file(&input);
    let _ = std::fs::remove_file(&output);
}

#[test]
fn test_e2e_unsupported_format_error_message() {
    let result = convert("document.odt");
    let err = result.unwrap_err();
    match err {
        ConvertError::UnsupportedFormat(ref ext) => {
            assert_eq!(ext, "odt", "Error should mention the unsupported extension");
        }
        _ => panic!("Expected UnsupportedFormat error, got {err:?}"),
    }
}

#[test]
fn test_e2e_missing_file_error() {
    let result = convert("nonexistent_document.docx");
    assert!(
        matches!(result.unwrap_err(), ConvertError::Io(_)),
        "Missing file should produce IO error"
    );
}

#[test]
fn test_e2e_docx_with_list_produces_pdf() {
    use std::io::Cursor;

    let abstract_num = docx_rs::AbstractNumbering::new(0).add_level(docx_rs::Level::new(
        0,
        docx_rs::Start::new(1),
        docx_rs::NumberFormat::new("bullet"),
        docx_rs::LevelText::new("•"),
        docx_rs::LevelJc::new("left"),
    ));
    let numbering = docx_rs::Numbering::new(1, 0);
    let nums = docx_rs::Numberings::new()
        .add_abstract_numbering(abstract_num)
        .add_numbering(numbering);

    let docx = docx_rs::Docx::new()
        .numberings(nums)
        .add_paragraph(
            docx_rs::Paragraph::new()
                .add_run(docx_rs::Run::new().add_text("Bullet 1"))
                .numbering(docx_rs::NumberingId::new(1), docx_rs::IndentLevel::new(0)),
        )
        .add_paragraph(
            docx_rs::Paragraph::new()
                .add_run(docx_rs::Run::new().add_text("Bullet 2"))
                .numbering(docx_rs::NumberingId::new(1), docx_rs::IndentLevel::new(0)),
        )
        .add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Regular text")),
        );

    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).unwrap();
    let data = cursor.into_inner();

    let result = convert_bytes(&data, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "Should produce valid PDF with list content"
    );
}

#[test]
fn test_normal_docx_has_no_warnings() {
    let docx_bytes = build_test_docx();
    let result = convert_bytes(&docx_bytes, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        result.warnings.is_empty(),
        "Normal DOCX should produce no warnings"
    );
}

#[test]
fn test_e2e_docx_with_header_footer_to_pdf() {
    use std::io::Cursor;

    let header = docx_rs::Header::new().add_paragraph(
        docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Document Title")),
    );
    let footer = docx_rs::Footer::new().add_paragraph(
        docx_rs::Paragraph::new().add_run(
            docx_rs::Run::new()
                .add_text("Page ")
                .add_field_char(docx_rs::FieldCharType::Begin, false)
                .add_instr_text(docx_rs::InstrText::PAGE(docx_rs::InstrPAGE::new()))
                .add_field_char(docx_rs::FieldCharType::Separate, false)
                .add_text("1")
                .add_field_char(docx_rs::FieldCharType::End, false),
        ),
    );
    let docx = docx_rs::Docx::new()
        .header(header)
        .footer(footer)
        .add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Body paragraph")),
        );
    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).unwrap();
    let data = cursor.into_inner();

    let result = convert_bytes(&data, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "DOCX with header/footer should produce valid PDF"
    );
}

#[test]
fn test_e2e_landscape_docx_to_pdf() {
    use std::io::Cursor;

    let docx = docx_rs::Docx::new()
        .page_size(16838, 11906)
        .page_orient(docx_rs::PageOrientationType::Landscape)
        .page_margin(
            docx_rs::PageMargin::new()
                .top(1440)
                .bottom(1440)
                .left(1440)
                .right(1440),
        )
        .add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Landscape document")),
        );
    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).unwrap();
    let data = cursor.into_inner();

    let result = convert_bytes(&data, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "Landscape DOCX should produce valid PDF"
    );
}

#[test]
fn test_docx_toc_pipeline_produces_pdf() {
    use std::io::Cursor;

    let toc = docx_rs::TableOfContents::new()
        .heading_styles_range(1, 3)
        .alias("Table of contents")
        .add_item(
            docx_rs::TableOfContentsItem::new()
                .text("Chapter 1")
                .toc_key("_Toc00000001")
                .level(1)
                .page_ref("2"),
        )
        .add_item(
            docx_rs::TableOfContentsItem::new()
                .text("Chapter 2")
                .toc_key("_Toc00000002")
                .level(1)
                .page_ref("5"),
        );

    let docx = docx_rs::Docx::new()
        .add_style(docx_rs::Style::new("Heading1", docx_rs::StyleType::Paragraph).name("Heading 1"))
        .add_table_of_contents(toc)
        .add_paragraph(
            docx_rs::Paragraph::new()
                .add_run(docx_rs::Run::new().add_text("Chapter 1"))
                .style("Heading1"),
        )
        .add_paragraph(
            docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text("Some body text")),
        );

    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor).unwrap();
    let data = cursor.into_inner();

    let result = convert_bytes(&data, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(
        result.as_pdf_bytes().unwrap().starts_with(b"%PDF"),
        "DOCX with TOC should produce valid PDF"
    );
}

// --- Raster (PNG/JPEG) output tests ---

use crate::config::OutputFormat;
use crate::error::OutputData;

#[test]
fn test_docx_to_png_produces_valid_png() {
    let docx = build_test_docx();
    let options = ConvertOptions {
        output_format: OutputFormat::Png,
        ..Default::default()
    };
    let result = convert_bytes(&docx, Format::Docx, &options).unwrap();
    let pages = result.output.as_raster_pages().expect("should be raster output");
    assert_eq!(pages.len(), 1, "single page DOCX → 1 PNG page");
    assert!(pages[0].starts_with(b"\x89PNG\r\n\x1a\n"), "valid PNG magic bytes");
}

#[test]
fn test_docx_to_jpeg_produces_valid_jpeg() {
    let docx = build_test_docx();
    let options = ConvertOptions {
        output_format: OutputFormat::Jpeg,
        jpeg_quality: 85,
        ..Default::default()
    };
    let result = convert_bytes(&docx, Format::Docx, &options).unwrap();
    let pages = result.output.as_raster_pages().expect("should be raster output");
    assert_eq!(pages.len(), 1);
    assert!(pages[0].starts_with(&[0xFF, 0xD8, 0xFF]), "valid JPEG magic bytes");
}

#[test]
fn test_xlsx_to_png_produces_valid_png() {
    let xlsx = build_test_xlsx();
    let options = ConvertOptions {
        output_format: OutputFormat::Png,
        ..Default::default()
    };
    let result = convert_bytes(&xlsx, Format::Xlsx, &options).unwrap();
    let pages = result.output.as_raster_pages().expect("should be raster output");
    assert!(!pages.is_empty(), "should have at least 1 page");
    assert!(pages[0].starts_with(b"\x89PNG\r\n\x1a\n"));
}

#[test]
fn test_pptx_to_jpeg_default_quality() {
    let pptx = build_test_pptx();
    let options = ConvertOptions {
        output_format: OutputFormat::Jpeg,
        ..Default::default()
    };
    let result = convert_bytes(&pptx, Format::Pptx, &options).unwrap();
    // JPEG quality defaults to 92 in ConvertOptions
    let pages = result.output.as_raster_pages().expect("should be raster output");
    assert!(!pages.is_empty());
    assert!(pages[0].starts_with(&[0xFF, 0xD8, 0xFF]));
}

#[test]
fn test_png_and_jpeg_for_same_document_both_valid() {
    let docx = build_test_docx();

    let png = convert_bytes(
        &docx,
        Format::Docx,
        &ConvertOptions {
            output_format: OutputFormat::Png,
            ..Default::default()
        },
    )
    .unwrap();
    let png_page = &png.output.as_raster_pages().unwrap()[0];
    assert!(png_page.starts_with(b"\x89PNG\r\n\x1a\n"), "valid PNG");
    assert!(!png_page.is_empty());

    let jpeg = convert_bytes(
        &docx,
        Format::Docx,
        &ConvertOptions {
            output_format: OutputFormat::Jpeg,
            jpeg_quality: 50,
            ..Default::default()
        },
    )
    .unwrap();
    let jpeg_page = &jpeg.output.as_raster_pages().unwrap()[0];
    assert!(jpeg_page.starts_with(&[0xFF, 0xD8, 0xFF]), "valid JPEG");
    assert!(!jpeg_page.is_empty());
}

#[test]
fn test_output_data_as_pdf_bytes_returns_none_for_raster() {
    let docx = build_test_docx();
    let options = ConvertOptions {
        output_format: OutputFormat::Png,
        ..Default::default()
    };
    let result = convert_bytes(&docx, Format::Docx, &options).unwrap();
    assert!(result.output.as_pdf_bytes().is_none());
    assert!(result.output.as_raster_pages().is_some());
}

#[test]
fn test_output_data_as_pdf_bytes_returns_some_for_pdf() {
    let docx = build_test_docx();
    let result = convert_bytes(&docx, Format::Docx, &ConvertOptions::default()).unwrap();
    assert!(result.output.as_pdf_bytes().is_some());
    assert!(result.output.as_raster_pages().is_none());
}

#[test]
fn test_convert_result_as_pdf_bytes_shorthand() {
    let docx = build_test_docx();
    let result = convert_bytes(&docx, Format::Docx, &ConvertOptions::default()).unwrap();
    assert_eq!(
        result.as_pdf_bytes(),
        result.output.as_pdf_bytes()
    );
}
