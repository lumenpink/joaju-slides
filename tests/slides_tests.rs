use joaju_slides::{
    parse_markdown_deck, render_handout_typst, render_notes_typst, render_slides_typst,
    SlideAspectRatio, SlidesError,
};
use std::path::PathBuf;

const SAMPLE_MARKDOWN: &str = r#"---
title: Fisiologia Cardiovascular - Ciclo Cardíaco
format: "16:9"
bibliography: "referencias.bib"
bib_style: "abnt"
---

![[diagrama_wiggers.svg]]
Fechamento Mitral e Fase Isovolumétrica

???
### Mecanismo Fisiológico
O fechamento da **valva mitral** marca o início da sístole ventricular.

---

# B3 (Terceira Bulha)
Sobrecarga de volume ventricular

???
A terceira bulha ocorre no início da diástole durante o enchimento ventricular rápido.
"#;

#[test]
fn test_parse_frontmatter_and_slides() {
    let deck = parse_markdown_deck(SAMPLE_MARKDOWN).expect("Deck parsing should succeed");

    assert_eq!(deck.title, "Fisiologia Cardiovascular - Ciclo Cardíaco");
    assert_eq!(deck.aspect_ratio, SlideAspectRatio::Widescreen16_9);
    assert_eq!(deck.bibliography, Some(PathBuf::from("referencias.bib")));
    assert_eq!(deck.bib_style, "abnt");
    assert_eq!(deck.slides.len(), 2);

    let s1 = &deck.slides[0];
    assert_eq!(s1.slide_index, 1);
    assert!(s1.visual_content.contains("Fechamento Mitral"));
    assert!(s1.speaker_notes.as_ref().unwrap().contains("Mecanismo Fisiológico"));

    let s2 = &deck.slides[1];
    assert_eq!(s2.slide_index, 2);
    assert_eq!(s2.title_hint.as_deref(), Some("B3 (Terceira Bulha)"));
    assert!(s2.visual_content.contains("Sobrecarga de volume"));
    assert!(s2.speaker_notes.as_ref().unwrap().contains("A terceira bulha ocorre"));
}

#[test]
fn test_render_slides_presentation_mode() {
    let deck = parse_markdown_deck(SAMPLE_MARKDOWN).unwrap();
    let typst = render_slides_typst(&deck).expect("Slides rendering should succeed");

    assert!(typst.contains("paper: \"presentation-16-9\""));
    assert!(typst.contains("Fechamento Mitral"));
    assert!(typst.contains("#pagebreak()"));
    // Speaker notes MUST NOT appear in slides mode
    assert!(!typst.contains("Mecanismo Fisiológico"));
    // Bibliography
    assert!(typst.contains("#bibliography(\"referencias.bib\", style: \"abnt\")"));
}

#[test]
fn test_render_handout_mode() {
    let deck = parse_markdown_deck(SAMPLE_MARKDOWN).unwrap();
    let typst = render_handout_typst(&deck).expect("Handout rendering should succeed");

    assert!(typst.contains("paper: \"a4\""));
    // Handout has the slide thumbnail box stroke and notes below
    assert!(typst.contains("stroke: 0.5pt + luma(180)"));
    assert!(typst.contains("Fechamento Mitral"));
    assert!(typst.contains("Mecanismo Fisiológico"));
    assert!(typst.contains("(Continuação das notas - Lâmina"));
}

#[test]
fn test_render_notes_mode() {
    let deck = parse_markdown_deck(SAMPLE_MARKDOWN).unwrap();
    let typst = render_notes_typst(&deck).expect("Notes rendering should succeed");

    assert!(typst.contains("paper: \"a4\""));
    // Visual content is omitted in notes mode, title becomes section header
    assert!(typst.contains("= B3 (Terceira Bulha)"));
    assert!(typst.contains("A terceira bulha ocorre"));
    assert!(!typst.contains("Sobrecarga de volume ventricular"));
}

#[test]
fn test_aspect_ratio_detection() {
    let md_4_3 = "---\ntitle: Test 4:3\nformat: \"4:3\"\n---\n# Slide 1";
    let deck_4_3 = parse_markdown_deck(md_4_3).unwrap();
    assert_eq!(deck_4_3.aspect_ratio, SlideAspectRatio::Standard4_3);

    let md_a4 = "---\ntitle: Test A4\nformat: \"a4-landscape\"\n---\n# Slide 1";
    let deck_a4 = parse_markdown_deck(md_a4).unwrap();
    assert_eq!(deck_a4.aspect_ratio, SlideAspectRatio::A4Landscape);
}

#[test]
fn test_orphan_notes_delimiter_fails_validation() {
    let broken_md = r#"---
title: Quebrado
---

???
Notas sem lâmina
"#;
    let res = parse_markdown_deck(broken_md);
    assert!(res.is_err());
    match res.unwrap_err() {
        SlidesError::OrphanNotesDelimiter { line } => assert_eq!(line, 5),
        other => panic!("Expected OrphanNotesDelimiter, got: {:?}", other),
    }
}

#[test]
fn test_duplicate_notes_delimiter_fails_validation() {
    let duplicate_notes_md = r#"---
title: Slide
---

# Titulo
???
Notas 1
???
Notas 2 duplicadas
"#;
    let res = parse_markdown_deck(duplicate_notes_md);
    assert!(res.is_err());
    match res.unwrap_err() {
        SlidesError::DuplicateNotesDelimiter { line } => assert_eq!(line, 8),
        other => panic!("Expected DuplicateNotesDelimiter, got: {:?}", other),
    }
}
