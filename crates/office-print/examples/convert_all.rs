use std::path::Path;

use office_print::config::{ConvertOptions, Format, OutputFormat};
use office_print::error::OutputData;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new("output");
    std::fs::create_dir_all(output_dir)?;

    let fixtures: Vec<(&str, Format)> = vec![
        ("tests/fixtures/xlsx/Formatting.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/date.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/WithChart.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/WithVariousData.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/merge_cells.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/temperature.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/right-to-left.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/Booleans.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/errors.xlsx", Format::Xlsx),
        ("tests/fixtures/xlsx/EmptySheet.xlsx", Format::Xlsx),
    ];

    let formats = vec![
        (OutputFormat::Pdf, "pdf"),
        (OutputFormat::Png, "png"),
        (OutputFormat::Jpeg, "jpg"),
    ];

    for (fixture_path, input_format) in &fixtures {
        let data = std::fs::read(fixture_path)?;
        let stem = Path::new(fixture_path)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        for (output_format, ext) in &formats {
            let options = ConvertOptions {
                output_format: *output_format,
                jpeg_quality: 92,
                ..Default::default()
            };

            let out_name = format!("{stem}.{ext}", stem = stem, ext = ext);
            let out_path = output_dir.join(&out_name);

            println!("Converting {} → {}...", fixture_path, out_name);

            match office_print::convert_bytes(&data, *input_format, &options) {
                Ok(result) => match &result.output {
                    OutputData::Pdf(pdf) => {
                        std::fs::write(&out_path, pdf)?;
                        println!("  ✓ {} bytes (PDF)", pdf.len());
                    }
                    OutputData::Raster { pages, format } => {
                        if pages.len() == 1 {
                            std::fs::write(&out_path, &pages[0])?;
                            println!("  ✓ {} bytes ({:?}, 1 page)", pages[0].len(), format);
                        } else {
                            // Multi-page raster: write page-1.png, page-2.png, ...
                            for (i, page) in pages.iter().enumerate() {
                                let page_name = format!("{stem}-page-{n}.{ext}", stem = stem, n = i + 1, ext = ext);
                                let page_path = output_dir.join(&page_name);
                                std::fs::write(&page_path, page)?;
                            }
                            println!(
                                "  ✓ {} pages, {} bytes total ({:?})",
                                pages.len(),
                                pages.iter().map(|p| p.len()).sum::<usize>(),
                                format
                            );
                        }
                    }
                },
                Err(e) => {
                    eprintln!("  ✗ Error: {e}");
                }
            }
        }
    }

    println!("\nDone. Output in: {}", output_dir.display());
    Ok(())
}
