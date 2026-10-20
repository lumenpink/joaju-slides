use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresentationMode {
    Slides,
    Handout,
    Notes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideAspectRatio {
    Widescreen16_9,
    Standard4_3,
    A4Landscape,
}

impl SlideAspectRatio {
    pub fn typst_paper(&self) -> &'static str {
        match self {
            Self::Widescreen16_9 => "presentation-16-9",
            Self::Standard4_3 => "presentation-4-3",
            Self::A4Landscape => "a4",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlideBlock {
    pub slide_index: usize,
    pub title_hint: Option<String>,
    pub visual_content: String,
    pub speaker_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlideDeck {
    pub title: String,
    pub aspect_ratio: SlideAspectRatio,
    pub bibliography: Option<PathBuf>,
    pub bib_style: String,
    pub slides: Vec<SlideBlock>,
}
