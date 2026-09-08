mod color;
mod markdown;
mod parser;
mod renderer;
mod tui;

use clap::Parser;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mmview")]
#[command(
    about = "Render Mermaid diagrams directly in the terminal as ASCII/Unicode art with colors",
    long_about = "Supports raw Mermaid diagrams, stdin streams, style/color directives, embedded Markdown blocks, and full interactive TUI mode with pan/zoom and live-reload."
)]
struct Args {
    /// Path to .mmd or .md file (reads from stdin if omitted)
    #[arg(value_name = "FILE")]
    file: Option<PathBuf>,

    /// Launch interactive TUI viewer (pan with hjkl, zoom/scroll, live file watch)
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// When reading Markdown, replace mermaid blocks inline and print the full document
    #[arg(short, long)]
    inline: bool,
}

fn render_diagram_str(input: &str) -> Result<String, String> {
    let diagram = parser::parse_mermaid(input)?;
    Ok(renderer::render_ascii(&diagram))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let content = match &args.file {
        Some(path) => fs::read_to_string(path)?,
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    if content.trim().is_empty() {
        eprintln!("Error: No input provided.");
        std::process::exit(1);
    }

    // Interactive TUI mode
    if args.interactive {
        return tui::run_tui(args.file, content);
    }

    // Non-interactive CLI stdout mode
    if content.contains("```mermaid") {
        if args.inline {
            let transformed = markdown::transform_markdown(&content, render_diagram_str);
            println!("{}", transformed);
        } else {
            let blocks = markdown::extract_mermaid_blocks(&content);
            for (i, block) in blocks.iter().enumerate() {
                if blocks.len() > 1 {
                    println!("--- Diagram #{} ---", i + 1);
                }
                match render_diagram_str(block) {
                    Ok(rendered) => println!("{}", rendered),
                    Err(err) => eprintln!("Diagram #{} Parse error: {}", i + 1, err),
                }
            }
        }
    } else {
        match render_diagram_str(&content) {
            Ok(rendered) => println!("{}", rendered),
            Err(err) => {
                eprintln!("Parse error: {}", err);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
