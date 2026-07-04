# docx4j

- **URL:** https://github.com/plutext/docx4j
- **Stars:** ~2.3k
- **Language:** Java
- **License:** Apache 2.0

## What it does

JAXB-based Java library for Word, PowerPoint, and Excel OOXML files. Supports DOCX-to-PDF via XSL-FO (Apache FOP). The most comprehensive open-source OOXML library in any language.

## Why relevant

- **Gold standard** for OOXML parsing — handles styles, numbering, sections, headers/footers, tables, images, and nearly all Word features.
- `docx4j-export-FO` module shows how to convert parsed OOXML into a layout format for PDF rendering — architecturally similar to our IR → Typst pipeline.
- Style resolution, numbering inheritance, and section property handling are directly applicable.

## Key files to study

- `org.docx4j.model.structure` — Document structure model
- `org.docx4j.convert.out.fo` — DOCX → XSL-FO conversion (analogous to our Typst codegen)
- `org.docx4j.wml` — Word Markup Language type definitions

## When to consult

- Debugging style resolution or numbering inheritance issues in DOCX parser
- Understanding complex table, section, or header/footer OOXML structures
