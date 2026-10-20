use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SlidesError {
    #[error("Markdown document contains no slides")]
    EmptyDeck,

    #[error("Orphan notes delimiter '???' found at line {line} without an active slide")]
    OrphanNotesDelimiter { line: usize },

    #[error("Duplicate notes delimiter '???' found at line {line} within the same slide")]
    DuplicateNotesDelimiter { line: usize },

    #[error("Invalid frontmatter: {0}")]
    InvalidFrontmatter(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Render error: {0}")]
    RenderError(String),
}
