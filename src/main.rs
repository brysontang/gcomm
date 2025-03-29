mod args;
mod git;
mod ollama;

use args::Args;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env::temp_dir;
use std::fs::write;
use std::process::Command;
use clap::Parser;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let staged_files = git::get_staged_files()?;
    let filtered_files: Vec<String> = staged_files
        .into_iter()
        .filter(|f| !git::is_large_or_binary(f))
        .collect();

    if filtered_files.is_empty() {
        println!("⚠️ No suitable staged files to generate commit message.");
        return Ok(());
    }

    let diff = git::get_filtered_diff(&filtered_files)?;

    if diff.trim().is_empty() {
        println!("⚠️ No meaningful changes detected in suitable files.");
        return Ok(());
    }

    let message = ollama::query_ollama(&diff, &args.model)?;
    println!("\n💡 Suggested Commit Message:\n\n{}", message);

    if args.yolo {
        let status = Command::new("git")
            .args(["commit", "-m", &message])
            .status()?;

        if status.success() {
            println!("✅ Commit created.");
        } else {
            eprintln!("❌ Failed to commit.");
        }
    } else if args.edit {
        let temp_path = temp_dir().join("gcommit_msg.txt");
        write(&temp_path, &message)?;

        let status = Command::new("git")
            .args(["commit", "-e", "-F"])
            .arg(temp_path.to_str().unwrap())
            .status()?;

        if !status.success() {
            eprintln!("❌ Commit aborted.");
        }
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(message.clone())?;
    println!("📋 Copied to clipboard!");

    Ok(())
}
