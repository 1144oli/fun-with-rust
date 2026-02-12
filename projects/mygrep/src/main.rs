use anyhow::{Context, Result};// for error handling
use clap::Parser;
use std::io::{self,Read};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let mut count:i32 = 0;
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

    //read lines
    for line in content.lines() {
        if line.contains(&args.pattern) {
            count += 1;
            println!("{} - {}",count, line);
        }
    }

    Ok(())
}
