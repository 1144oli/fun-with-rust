use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Read};
use std::path::PathBuf;
use boxy_cli::prelude::*;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    path: Option<PathBuf>,

    /// Optional output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let mut count: i32 = 0;
    let args = Cli::parse();

    let content = match &args.path {
        Some(path) => {
            std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path.display()))?
        }
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("failed to read from stdin")?;
            buffer
        }
    };

    // Collect matches
    let mut matches = Vec::new();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            count += 1;
            matches.push(format!("{} - {}", count, line));
        }
    }

    if let Some(path) = args.output {
        std::fs::write(&path, matches.join("\n"))
            .with_context(|| format!("could not write to `{}`", path.display()))?;
    } 

    else {
        for line in &matches {
            Boxy::builder()
                .box_type(BoxType::Single)
                .color("#00ffff")
                .add_segment(line, "#ffffff", BoxAlign::Center)
                .build()
                .display();
        }

        Boxy::builder()
            .box_type(BoxType::Single)
            .color("#44ff00")
            .add_segment(&format!("Total: {}", count), "#ffffff", BoxAlign::Left)
            .external_padding(BoxPad::uniform(5))
            .build()
            .display();
    }

    Ok(())
}
