use crate::error::SlidesError;
use crate::model::{SlideAspectRatio, SlideBlock, SlideDeck};
use std::path::PathBuf;

pub fn parse_markdown_deck(content: &str) -> Result<SlideDeck, SlidesError> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(SlidesError::EmptyDeck);
    }

    let mut title = "Apresentação".to_string();
    let mut aspect_ratio = SlideAspectRatio::Widescreen16_9;
    let mut bibliography: Option<PathBuf> = None;
    let mut bib_style = "abnt".to_string();

    let mut start_idx = 0;

    // Check frontmatter
    if !lines.is_empty() && lines[0].trim() == "---" {
        let mut fm_end = None;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                fm_end = Some(i);
                break;
            }
        }

        if let Some(end) = fm_end {
            for line in &lines[1..end] {
                let trimmed = line.trim();
                if let Some((k, v)) = trimmed.split_once(':') {
                    let key = k.trim();
                    let val = v.trim().trim_matches('"').trim_matches('\'');
                    match key {
                        "title" => title = val.to_string(),
                        "format" => match val {
                            "4:3" => aspect_ratio = SlideAspectRatio::Standard4_3,
                            "a4-landscape" => aspect_ratio = SlideAspectRatio::A4Landscape,
                            _ => aspect_ratio = SlideAspectRatio::Widescreen16_9,
                        },
                        "bibliography" => bibliography = Some(PathBuf::from(val)),
                        "bib_style" => bib_style = val.to_string(),
                        _ => {}
                    }
                }
            }
            start_idx = end + 1;
        }
    }

    // Now parse slides delimited by `---` and notes by `???`
    let mut slides = Vec::new();
    let mut current_visual_lines: Vec<&str> = Vec::new();
    let mut current_notes_lines: Option<Vec<&str>> = None;
    let mut slide_started = false;

    let mut line_no = start_idx + 1;
    let mut i = start_idx;

    while i < lines.len() {
        let raw_line = lines[i];
        let trimmed = raw_line.trim();

        if trimmed == "---" {
            if slide_started {
                let block = finalize_slide(
                    slides.len() + 1,
                    &current_visual_lines,
                    current_notes_lines.as_deref(),
                );
                slides.push(block);
                current_visual_lines.clear();
                current_notes_lines = None;
                slide_started = false;
            }
        } else if trimmed == "???" {
            if !slide_started && current_visual_lines.is_empty() {
                return Err(SlidesError::OrphanNotesDelimiter { line: line_no });
            }
            if current_notes_lines.is_some() {
                return Err(SlidesError::DuplicateNotesDelimiter { line: line_no });
            }
            slide_started = true;
            current_notes_lines = Some(Vec::new());
        } else {
            if !trimmed.is_empty() || slide_started {
                slide_started = true;
            }
            if let Some(ref mut notes) = current_notes_lines {
                notes.push(raw_line);
            } else if slide_started {
                current_visual_lines.push(raw_line);
            }
        }

        i += 1;
        line_no += 1;
    }

    if slide_started || !current_visual_lines.is_empty() || current_notes_lines.is_some() {
        let block = finalize_slide(
            slides.len() + 1,
            &current_visual_lines,
            current_notes_lines.as_deref(),
        );
        slides.push(block);
    }

    if slides.is_empty() {
        return Err(SlidesError::EmptyDeck);
    }

    Ok(SlideDeck {
        title,
        aspect_ratio,
        bibliography,
        bib_style,
        slides,
    })
}

fn finalize_slide(
    slide_index: usize,
    visual_lines: &[&str],
    notes_lines: Option<&[&str]>,
) -> SlideBlock {
    let visual_content = visual_lines.join("\n").trim().to_string();
    let speaker_notes = notes_lines.map(|lines| lines.join("\n").trim().to_string());

    let mut title_hint = None;
    for line in visual_lines {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix('#') {
            title_hint = Some(heading.trim_start_matches('#').trim().to_string());
            break;
        }
    }

    SlideBlock {
        slide_index,
        title_hint,
        visual_content,
        speaker_notes,
    }
}
