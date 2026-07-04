# texmath

- **URL:** https://github.com/jgm/texmath
- **Stars:** ~390
- **Language:** Haskell
- **License:** GPL-2.0

## What it does

Converts between LaTeX math, MathML, OMML, Typst math, GNU eqn, and Pandoc's native format. Used internally by Pandoc for all math conversion.

## Why relevant

- **The single most relevant math conversion project.** Has both an OMML reader and a Typst math writer — exactly the pipeline office-print needs.
- `Text.TeXMath.Readers.OMML` parses OMML (the math format inside DOCX files).
- `Text.TeXMath.Writers.Typst` serializes to Typst math notation.
- Symbol mappings and conversion rules are directly portable to our `omml.rs`.

## Key files to study

- `src/Text/TeXMath/Readers/OMML.hs` — OMML parsing
- `src/Text/TeXMath/Writers/Typst.hs` — Typst math output
- `src/Text/TeXMath/Types.hs` — Internal math IR (Exp type)
- `src/Text/TeXMath/Shared.hs` — Shared symbol mappings

## When to consult

- Improving OMML → Typst math conversion quality
- Adding support for new math constructs (matrices, accents, etc.)
- Cross-checking symbol mapping correctness
