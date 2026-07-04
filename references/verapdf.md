# veraPDF

- **URL:** https://github.com/veraPDF/veraPDF-library
- **Stars:** ~316
- **Language:** Java
- **License:** MPL-2.0 / GPL-3.0

## What it does

Industry-standard open-source PDF/A and PDF/UA validator. Supports PDF/A-1 through PDF/A-4 and PDF/UA-1.

## Why relevant

- **Validation tool** for ensuring office-print's PDF/A and PDF/UA output is compliant.
- Can be integrated into CI/CD to automatically validate generated PDFs.
- Detailed validation reports help diagnose specific compliance issues.

## Usage

```bash
# Validate PDF/A compliance
verapdf --flavour 2b output.pdf

# Validate PDF/UA compliance
verapdf --flavour ua1 output.pdf
```

## When to consult

- Validating PDF/A-2b or PDF/UA-1 compliance of generated PDFs
- Diagnosing specific PDF standard compliance failures
