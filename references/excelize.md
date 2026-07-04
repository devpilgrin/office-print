# excelize

- **URL:** https://github.com/qax-os/excelize
- **Stars:** ~20.4k
- **Language:** Go
- **License:** BSD-3-Clause

## What it does

Go library for reading and writing XLSX/XLSM/XLTX/XLTM. Supports streaming API, charts, images, conditional formatting, merged cells, data validation, pivot tables — nearly every Excel feature.

## Why relevant

- **Most complete** XLSX implementation in any language (by feature coverage).
- Conditional formatting rule handling is among the best — DataBar, ColorScale, IconSet.
- Merged cell rendering, column width calculation, and style resolution are thoroughly implemented.
- Clean Go code that is straightforward to read.

## Key files to study

- `cell.go` — Cell value reading and type resolution
- `styles.go` — Style resolution and number format handling
- `condFmt.go` (or similar) — Conditional formatting rules
- `merge.go` — Merged cell handling

## When to consult

- Implementing XLSX conditional formatting (DataBar, IconSet)
- Debugging merged cell resolution or column width calculation
- Understanding XLSX number format strings
