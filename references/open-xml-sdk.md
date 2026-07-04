# Open-XML-SDK

- **URL:** https://github.com/dotnet/Open-XML-SDK
- **Stars:** ~4.4k
- **Language:** C#
- **License:** MIT

## What it does

Microsoft's official SDK for working with Office Open XML documents. Strongly-typed APIs for all OOXML elements — Word, Excel, and PowerPoint.

## Why relevant

- **Canonical reference** for OOXML types and relationships. When in doubt about what an OOXML element means or how it's structured, this is authoritative.
- Type hierarchy directly mirrors the ECMA-376 spec.
- Useful for cross-checking our parser's interpretation of OOXML attributes.

## Key files to study

- `DocumentFormat.OpenXml/GeneratedCode/` — Auto-generated types for all OOXML elements
- Type definitions for `WordprocessingML`, `SpreadsheetML`, `PresentationML`

## When to consult

- Verifying correct parsing of OOXML attributes and element structures
- Understanding element type relationships in the OOXML schema
- Cross-referencing with ECMA-376 spec
