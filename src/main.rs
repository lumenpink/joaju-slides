use clap::Parser;
use joaju_slides::{
    parse_markdown_deck, render_handout_typst, render_notes_typst, render_slides_typst,
    validate_deck, PresentationMode,
};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "joaju-slides",
    about = "Presentation slides, handouts, and speaker notes generator from Markdown via Typst",
    version = joaju_slides::VERSION.trim()
)]
struct Cli {
    /// Markdown source file
    input: PathBuf,

    /// Generate presentation slides (16:9 / 4:3)
    #[arg(long, conflicts_with_all = ["handout", "notes"])]
    slides: bool,

    /// Generate handout / apostila (A4 with slide thumbnail and notes)
    #[arg(long, conflicts_with_all = ["slides", "notes"])]
    handout: bool,

    /// Generate continuous speaker notes / roteiro
    #[arg(long, conflicts_with_all = ["slides", "handout"])]
    notes: bool,

    /// Output Typst file (prints to stdout if omitted)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let content = match fs::read_to_string(&cli.input) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Error reading '{}': {}", cli.input.display(), err);
            return ExitCode::FAILURE;
        }
    };

    let deck = match parse_markdown_deck(&content) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Parse error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let warnings = validate_deck(&deck);
    for warning in warnings {
        eprintln!("Warning: {:?}", warning);
    }

    let mode = if cli.handout {
        PresentationMode::Handout
    } else if cli.notes {
        PresentationMode::Notes
    } else {
        // Default to slides
        PresentationMode::Slides
    };

    let result = match mode {
        PresentationMode::Slides => render_slides_typst(&deck),
        PresentationMode::Handout => render_handout_typst(&deck),
        PresentationMode::Notes => render_notes_typst(&deck),
    };

    let rendered = match result {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Rendering error: {}", err);
            return ExitCode::FAILURE;
        }
    };

    if let Some(out_path) = cli.output {
        if let Err(err) = fs::write(&out_path, rendered) {
            eprintln!("Error writing '{}': {}", out_path.display(), err);
            return ExitCode::FAILURE;
        }
    } else {
        print!("{}", rendered);
    }

    ExitCode::SUCCESS
}
