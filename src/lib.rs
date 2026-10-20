//! Presentation slides, handouts, and speaker notes generator from Markdown via Typst.

pub mod error;
pub mod model;
pub mod parser;
pub mod renderer;
pub mod validator;

pub use error::SlidesError;
pub use model::{PresentationMode, SlideAspectRatio, SlideBlock, SlideDeck};
pub use parser::parse_markdown_deck;
pub use renderer::{render_handout_typst, render_notes_typst, render_slides_typst};
pub use validator::{validate_deck, SlideWarning};

pub const VERSION: &str = include_str!("../VERSION");
