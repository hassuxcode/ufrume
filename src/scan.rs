use audiotags::Tag;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub track: Option<u16>,
}

pub fn scan_for_music(
    input_dir: &PathBuf,
) -> Result<Vec<(PathBuf, AudioMetadata)>, Box<dyn std::error::Error>> {
    let music_extensions = ["mp3", "flac", "m4a", "wav", "ogg", "aac"];

    let music_file_paths: Vec<PathBuf> = WalkDir::new(input_dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if music_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            return Some(path.to_path_buf());
                        }
                    }
                }
            }
            None
        })
        .collect();

    if music_file_paths.is_empty() {
        return Ok(Vec::new());
    }

    let thread_count = rayon::current_num_threads();
    println!(
        "  Processing {} files using {} threads",
        music_file_paths.len(),
        thread_count
    );

    let start_time = Instant::now();

    let pb = Arc::new(ProgressBar::new(music_file_paths.len() as u64));
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{bar:40.cyan/blue}] {pos}/{len} [{elapsed_precise}] {msg}")?
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );

    let successful_extractions = Arc::new(Mutex::new(0));
    let failed_extractions = Arc::new(Mutex::new(0));

    let results: Vec<Option<(PathBuf, AudioMetadata)>> = music_file_paths
        .par_iter()
        .map(|path| {
            if let Some(filename) = path.file_name() {
                pb.set_message(filename.to_string_lossy().to_string());
            }

            match extract_metadata(path) {
                Ok(metadata) => {
                    *successful_extractions.lock().unwrap() += 1;
                    pb.inc(1);
                    Some((path.clone(), metadata))
                }
                Err(err) => {
                    eprintln!(
                        "  Failed to extract metadata from {}: {}",
                        path.display(),
                        err
                    );
                    *failed_extractions.lock().unwrap() += 1;
                    pb.inc(1);
                    None
                }
            }
        })
        .collect();

    pb.finish_and_clear();

    let duration = start_time.elapsed();

    let music_files: Vec<(PathBuf, AudioMetadata)> =
        results.into_iter().filter_map(|r| r).collect();

    let failed_count = *failed_extractions.lock().unwrap();
    if failed_count > 0 {
        println!(
            "  {} files processed, {} failed in {:.2}s",
            music_files.len(),
            failed_count,
            duration.as_secs_f64()
        );
    } else {
        println!(
            "  {} files processed in {:.2}s",
            music_files.len(),
            duration.as_secs_f64()
        );
    }

    Ok(music_files)
}

fn extract_metadata(path: &Path) -> Result<AudioMetadata, Box<dyn std::error::Error>> {
    let tag = Tag::default().read_from_path(path)?;

    Ok(AudioMetadata {
        title: tag.title().map(str::to_string),
        artist: tag
            .artists()
            .and_then(|artists| artists.first().map(|s| s.to_string())),
        album: tag.album_title().map(str::to_string),
        album_artist: tag.album_artist().map(|s| extract_first_artist(s)),
        year: tag.year(),
        genre: tag.genre().map(str::to_string),
        track: tag.track().0.map(|t| t as u16),
    })
}

fn extract_first_artist(artist_string: &str) -> String {
    let delimiters = [
        ", ", " & ", " and ", " feat. ", " feat ", " ft. ", " ft ", " x ", " X ", " vs ", " vs. ",
        " with ", " + ", " / ",
    ];

    let mut earliest_pos = artist_string.len();

    for delimiter in &delimiters {
        if let Some(pos) = artist_string.find(delimiter) {
            if pos < earliest_pos {
                earliest_pos = pos;
            }
        }
    }

    if earliest_pos == artist_string.len() {
        artist_string.trim().to_string()
    } else {
        artist_string[..earliest_pos].trim().to_string()
    }
}
