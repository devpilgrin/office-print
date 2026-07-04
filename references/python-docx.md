# python-docx

- **URL:** https://github.com/python-openxml/python-docx
- **Stars:** ~5.5k
- **Language:** Python
- **License:** MIT

## What it does

Create and modify Word 2007+ (.docx) files. Well-documented object model for paragraphs, runs, tables, styles, sections, headers/footers, and images.

## Why relevant

- Best-documented DOCX object model — clear mapping between OOXML elements and user-facing concepts.
- Style inheritance resolution code is well-structured and readable.
- Excellent companion to the ECMA-376 spec for understanding "what this XML actually means."

## Key files to study

- `docx/oxml/text/paragraph.py` — Paragraph element parsing
- `docx/oxml/table.py` — Table structure
- `docx/styles/` — Style resolution chain

## When to consult

- Understanding DOCX object model and style hierarchy
- Mapping OOXML elements to semantic document concepts
