"""
office-print Python wrapper — convert Office documents via CLI subprocess.

Requires `office-print.exe` on PATH or in the same directory.
"""
from __future__ import annotations

import subprocess
import shutil
from pathlib import Path
from typing import Optional, Sequence


def _find_binary() -> str:
    """Locate the office-print binary."""
    exe = shutil.which("office-print") or shutil.which("office-print.exe")
    if exe:
        return exe
    # Fallback: same directory as this module
    local = Path(__file__).resolve().parent / "office-print.exe"
    if local.exists():
        return str(local)
    raise FileNotFoundError(
        "office-print binary not found. Install it or place office-print.exe "
        "next to office_print.py"
    )


def convert(
    input_file: str | Path,
    output_file: Optional[str | Path] = None,
    *,
    output_dir: Optional[str | Path] = None,
    format: str = "pdf",            # "pdf" | "png" | "jpeg"
    jpeg_quality: int = 92,
    paper: Optional[str] = None,     # "a4" | "letter" | "legal"
    landscape: bool = False,
    font_paths: Optional[Sequence[str | Path]] = None,
    sheets: Optional[str] = None,    # XLSX: "Sheet1,Data"
    slides: Optional[str] = None,    # PPTX: "1-5" or "3"
    jobs: int = 0,                   # 0 = all CPU cores
    streaming: bool = False,
    pdf_a: bool = False,
    tagged: bool = False,
    pdf_ua: bool = False,
) -> subprocess.CompletedProcess:
    """
    Convert an Office document to PDF/PNG/JPEG.

    Returns the CompletedProcess. Check `result.returncode == 0` for success.
    """
    binary = _find_binary()
    cmd = [binary, str(input_file)]

    if output_file:
        cmd += ["-o", str(output_file)]
    if output_dir:
        cmd += ["--outdir", str(output_dir)]
    if format != "pdf":
        cmd += ["--format", format]
    if format == "jpeg":
        cmd += ["--jpeg-quality", str(jpeg_quality)]
    if paper:
        cmd += ["--paper", paper]
    if landscape:
        cmd += ["--landscape"]
    if font_paths:
        for fp in font_paths:
            cmd += ["--font-path", str(fp)]
    if sheets:
        cmd += ["--sheets", sheets]
    if slides:
        cmd += ["--slides", slides]
    if jobs:
        cmd += ["-j", str(jobs)]
    if streaming:
        cmd += ["--streaming"]
    if pdf_a:
        cmd += ["--pdf-a"]
    if tagged:
        cmd += ["--tagged"]
    if pdf_ua:
        cmd += ["--pdf-ua"]

    return subprocess.run(cmd, capture_output=True, text=True)


def convert_bytes(
    data: bytes,
    suffix: str = ".docx",
    output_file: Optional[str | Path] = None,
    **kwargs,
) -> subprocess.CompletedProcess:
    """
    Convert bytes to PDF/PNG/JPEG via a temp file.

    `suffix` must match the input format: ".docx", ".xlsx", or ".pptx".
    """
    import tempfile
    with tempfile.NamedTemporaryFile(suffix=suffix, delete=False) as tmp:
        tmp.write(data)
        tmp_path = tmp.name

    try:
        return convert(tmp_path, output_file=output_file, **kwargs)
    finally:
        Path(tmp_path).unlink(missing_ok=True)


# ---------------------------------------------------------------------------
# PDF utilities (requires office-print built with `pdf-ops` feature)
# ---------------------------------------------------------------------------


def merge_pdfs(
    inputs: Sequence[str | Path],
    output: str | Path,
) -> subprocess.CompletedProcess:
    """Merge multiple PDF files into one."""
    binary = _find_binary()
    cmd = [binary, "merge", "-o", str(output), *[str(p) for p in inputs]]
    return subprocess.run(cmd, capture_output=True, text=True)


def split_pdf(
    input_pdf: str | Path,
    output_dir: str | Path,
    ranges: Sequence[str],  # e.g. ["1-3", "5", "7-"]
) -> subprocess.CompletedProcess:
    """Split a PDF into parts by page ranges."""
    binary = _find_binary()
    cmd = [binary, "split", str(input_pdf), "--outdir", str(output_dir)]
    for r in ranges:
        cmd += ["--pages", r]
    return subprocess.run(cmd, capture_output=True, text=True)
