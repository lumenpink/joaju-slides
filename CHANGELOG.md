# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- (renderer) ensure typst 0.15 syntax compatibility for handout and notes (closes #2).

### Added
- Presentation slides, handouts, and speaker notes generator from Markdown via Typst (closes #1).
- Delimiter support for slides (`---`) and speaker notes (`???`).
- Triple output modes: `--slides` (16:9/4:3), `--handout` (A4 with thumbnails), and `--notes` (continuous A4).
- Automatic bibliography integration via Typst `#bibliography()`.
- Overflow validation and error diagnostics (`ERR_ORPHAN_NOTES_DELIMITER`, `ERR_DUPLICATE_NOTES_DELIMITER`, `WARN_SLIDE_OVERFLOW`, `WARN_HANDOUT_OVERFLOW`).
- Inicialização do repositório em conformidade com o padrão GOST.
