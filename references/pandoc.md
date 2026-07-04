# pandoc

- **URL:** https://github.com/jgm/pandoc
- **Stars:** ~42.4k
- **Language:** Haskell
- **License:** GPL-2.0

## What it does

Universal markup converter. Parses input formats (including DOCX) into an internal AST, then serializes to output formats (including Typst). Supports custom filters on the AST.

## Why relevant

- **Gold standard** for IR-based document conversion — the same architecture as office-print.
- DOCX reader (`Readers/Docx.hs`) is a battle-tested parser showing which OOXML features matter.
- Typst writer (`Writers/Typst.hs`) is the most mature IR → Typst code generator — directly analogous to our `typst_gen.rs`.
- AST design (`Pandoc Meta [Block]`) is a proven reference for format-agnostic document IR.

## Key files to study

- `src/Text/Pandoc/Readers/Docx.hs` — DOCX → AST parsing
- `src/Text/Pandoc/Writers/Typst.hs` — AST → Typst codegen
- `src/Text/Pandoc/Definition.hs` — The IR definition (Block, Inline types)

## When to consult

- Designing IR improvements or new element types
- Implementing new Typst codegen features (table, math, list rendering)
- Cross-checking DOCX parsing behavior
