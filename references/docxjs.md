# docxjs (docx-preview)

- **URL:** https://github.com/VolodymyrBaydalka/docxjs
- **Stars:** ~1.9k
- **Language:** TypeScript
- **License:** MIT

## What it does

DOCX rendering library that converts DOCX documents to HTML while preserving visual layout. Published on npm as `docx-preview`.

## Why relevant

- **Closest analog** to what office-print does — DOCX to visual output, without LibreOffice.
- Shows how to resolve styles, handle sections, page layout, headers/footers, tables, and images for visual rendering.
- TypeScript source is readable and well-structured.

## Key files to study

- `src/document-parser.ts` — DOCX XML parsing
- `src/html/` — Element-to-HTML rendering (analogous to our Typst codegen)
- Style resolution logic

## When to consult

- Comparing our rendering approach with another DOCX-to-visual-output implementation
- Debugging visual fidelity issues (margins, page breaks, table layout)
