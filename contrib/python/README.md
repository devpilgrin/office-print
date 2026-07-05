# Python integration for office-print

## Quick start

```python
from office_print import convert, convert_bytes

# File conversion
result = convert("report.docx", "report.pdf")
assert result.returncode == 0

# Bytes conversion (in-memory)
with open("template.docx", "rb") as f:
    docx_bytes = f.read()
result = convert_bytes(docx_bytes, ".docx", "output.pdf")
```

## API

### `convert(input_file, output_file=None, **kwargs)`

Convert a file. Returns `subprocess.CompletedProcess`.

| Arg | Type | Default | Description |
|-----|------|---------|-------------|
| `input_file` | `str \| Path` | — | Input `.docx`, `.xlsx`, or `.pptx` |
| `output_file` | `str \| Path` | `None` | Output path (auto-named if `None`) |
| `output_dir` | `str \| Path` | `None` | Output directory for batch conversion |
| `format` | `str` | `"pdf"` | `"pdf"`, `"png"`, or `"jpeg"` |
| `jpeg_quality` | `int` | `92` | JPEG quality 1–100 |
| `paper` | `str` | `None` | `"a4"`, `"letter"`, `"legal"` |
| `landscape` | `bool` | `False` | Force landscape orientation |
| `font_paths` | `list[str\|Path]` | `None` | Extra font directories |
| `sheets` | `str` | `None` | XLSX sheets: `"Sheet1,Data"` |
| `slides` | `str` | `None` | PPTX slide range: `"1-5"` |
| `jobs` | `int` | `0` | Parallel jobs (0 = all cores) |
| `streaming` | `bool` | `False` | Streaming mode for large XLSX |
| `pdf_a` | `bool` | `False` | PDF/A-2b archival output |
| `tagged` | `bool` | `False` | Tagged PDF (accessibility) |
| `pdf_ua` | `bool` | `False` | PDF/UA-1 (implies `tagged`) |

### `convert_bytes(data, suffix=".docx", output_file=None, **kwargs)`

Convert bytes via temp file. Same kwargs as `convert()`.

### `merge_pdfs(inputs, output)`

Merge multiple PDFs into one.

### `split_pdf(input_pdf, output_dir, ranges)`

Split PDF by page ranges. `ranges = ["1-3", "5", "7-"]`.

## Installation

```
pip install office_print.py
```

Or copy `office_print.py` into your project — zero dependencies.

## Requirements

- `office-print` binary on PATH or next to `office_print.py`
- Downloaded from: https://github.com/devpilgrin/office-print/releases
