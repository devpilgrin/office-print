# rustybuzz

- **URL:** https://github.com/harfbuzz/rustybuzz
- **Stars:** ~589
- **Language:** Rust
- **License:** MIT

## What it does

Pure Rust port of HarfBuzz's text shaping algorithm. Passes 2221 of 2252 HarfBuzz shaping tests. Handles complex scripts (Arabic, Devanagari, Thai, CJK).

## Why relevant

- **Used by Typst** for text shaping. Understanding its capabilities and limitations is key for diagnosing text rendering issues.
- Complex script support (RTL, ligatures, contextual alternates) affects how office-print's output looks for non-Latin text.
- Shaping failures or limitations in rustybuzz directly affect our PDF output quality.

## When to consult

- Debugging text rendering issues (wrong ligatures, broken Arabic/CJK text)
- Understanding font feature support (OpenType features, variable fonts)
- Investigating text shaping performance
