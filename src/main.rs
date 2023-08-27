#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, BufRead, BufWriter, Write};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("ファイルが読み込めませんでした");
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())
}

// Result enum を利用したエラーハンドリングを考慮した場合の例
// fn main() -> Result<()> {
//     let args = Cli::parse();

//     let content = std::fs::read_to_string(&args.path)
//         .with_context(|| format!("could not read file `{}`", args.path.display()))?;

//     grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

//     Ok(())
// }
