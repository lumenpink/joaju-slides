use crate::model::SlideDeck;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideWarning {
    WarnSlideOverflow {
        slide_index: usize,
        line_count: usize,
    },
    WarnHandoutOverflow {
        slide_index: usize,
        char_count: usize,
    },
}

pub fn validate_deck(deck: &SlideDeck) -> Vec<SlideWarning> {
    let mut warnings = Vec::new();

    for slide in &deck.slides {
        let lines = slide.visual_content.lines().count();
        if lines > 15 || slide.visual_content.len() > 600 {
            warnings.push(SlideWarning::WarnSlideOverflow {
                slide_index: slide.slide_index,
                line_count: lines,
            });
        }

        if let Some(ref notes) = slide.speaker_notes {
            if notes.len() > 1500 {
                warnings.push(SlideWarning::WarnHandoutOverflow {
                    slide_index: slide.slide_index,
                    char_count: notes.len(),
                });
            }
        }
    }

    warnings
}
