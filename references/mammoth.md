# mammoth.js

- **URL:** https://github.com/mwilliamson/mammoth.js
- **Stars:** ~6.1k
- **Language:** JavaScript (also available in Python and Java)
- **License:** BSD-2-Clause

## What it does

Converts DOCX files to clean, semantic HTML. Maps Word's semantic styles (Heading 1 → `<h1>`) rather than replicating exact visual formatting.

## Why relevant

- Clean separation between DOCX parsing and output generation.
- Style-mapping DSL shows which OOXML semantic concepts matter most.
- The approach of mapping styles to output elements is directly applicable to our IR → Typst pipeline.

## Key files to study

- `lib/docx/` — DOCX XML walking and element extraction
- `lib/documents.js` — Internal document model (the IR)
- `lib/html/` — IR → HTML generation

## When to consult

- Simplifying style mapping logic
- Understanding which DOCX semantic structures to preserve vs. discard
