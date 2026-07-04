# python-pptx

- **URL:** https://github.com/scanny/python-pptx
- **Stars:** ~3.2k
- **Language:** Python
- **License:** MIT

## What it does

Create, read, and update PowerPoint (.pptx) files. Includes detailed analysis docs for slide layouts, masters, and placeholder inheritance.

## Why relevant

- **Essential reference** for PPTX slide master → layout → slide inheritance chain.
- `docs/dev/analysis/` contains rare deep-dives into how PowerPoint resolves placeholders, colors, fonts, and backgrounds across the inheritance hierarchy.
- The analysis docs are more useful than the code itself — they reverse-engineer PowerPoint's actual behavior.

## Key files to study

- `docs/dev/analysis/sld-layout.rst` — Slide layout inheritance
- `docs/dev/analysis/placeholders/` — Placeholder resolution rules
- `pptx/oxml/shapes/` — Shape XML parsing

## When to consult

- Debugging PPTX slide master/layout inheritance
- Understanding placeholder resolution and theme color application
- Resolving shape property inheritance (fill, line, text body defaults)
