use crate::{config::load_or_create_config, organize::organize_music_files, scan::scan_for_music};
use clap::Parser;
use console::style;
use std::path::PathBuf;

mod config;
mod organize;
mod scan;

#[derive(Parser)]
#[command(name = "organisiert")]
#[command(about = "Multithreaded CLI tool to organize music files into a folder structure defined by you")]
#[command(author = "PandaDEV, contact@pandadev.net")]
#[command(version = "1.0.0")]
struct Cli {
    input_dir: PathBuf,
    output_dir: PathBuf,
    #[arg(short, long)]
    move_files: bool,
    #[arg(short, long)]
    threads: Option<usize>,
    #[arg(short, long)]
    verbose: bool,
}

fn verify_paths(input_dir: &PathBuf, output_dir: &PathBuf) -> Result<(), String> {
    if !input_dir.exists() {
        return Err(format!(
            "Input path does not exist: {}",
            input_dir.display()
        ));
    }

    if !output_dir.exists() {
        return Err(format!(
            "Output path does not exist: {}",
            output_dir.display()
        ));
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    println!(
        "{} {}",
        style("[1/4]").bold().dim(),
        "Loading configuration..."
    );
    let config = match load_or_create_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("ERROR: Failed to load config: {}", e);
            std::process::exit(1)
        }
    };

    println!("{} {}", style("[2/4]").bold().dim(), "Verifying paths...");
    if let Err(e) = verify_paths(&cli.input_dir, &cli.output_dir) {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }

    println!("  Input:  {}", style(cli.input_dir.display()).green());
    println!("  Output: {}", style(cli.output_dir.display()).green());

    if cli.move_files {
        println!("  Mode:   {}", style("Move").yellow());
    } else {
        println!("  Mode:   {}", style("Copy").cyan());
    }

    if let Some(count) = cli.threads {
        if count == 0 {
            eprintln!("ERROR: Thread count must be greater than 0");
            std::process::exit(1);
        }

        rayon::ThreadPoolBuilder::new()
            .num_threads(count)
            .build_global()
            .map_err(|e| {
                eprintln!("ERROR: Failed to configure thread pool: {}", e);
                std::process::exit(1);
            })
            .unwrap();
        println!("  Threads: {}", style(count.to_string()).cyan());
    }

    println!(
        "{} {}",
        style("[3/4]").bold().dim(),
        "Scanning music files..."
    );

    let music_files = match scan_for_music(&cli.input_dir) {
        Ok(music_files) => {
            if music_files.is_empty() {
                println!("No music files found to organize.");
                return;
            } else {
                if cli.verbose {
                    println!("\nScan Results:");
                    for (_i, (path, metadata)) in music_files.iter().enumerate().take(5) {
                        println!(
                            "  {} - {} - {}",
                            metadata.artist.as_deref().unwrap_or("Unknown Artist"),
                            metadata.title.as_deref().unwrap_or("Unknown Title"),
                            style(path.file_name().unwrap_or_default().to_string_lossy()).dim()
                        );
                    }
                    if music_files.len() > 5 {
                        println!("  ... and {} more files", music_files.len() - 5);
                    }
                }
                music_files
            }
        }
        Err(e) => {
            eprintln!("ERROR: Failed to scan music files: {}", e);
            std::process::exit(1);
        }
    };

    println!(
        "\n{} {}",
        style("[4/4]").bold().dim(),
        "Organizing music files..."
    );

    match organize_music_files(&music_files, &cli.output_dir, &config, cli.move_files) {
        Ok(_result) => (),
        Err(e) => {
            eprintln!("ERROR: Failed to organize music files: {}", e);
            std::process::exit(1);
        }
    }
}
