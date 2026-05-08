use std::{
    fs,
    path::{Path, PathBuf},
};

use tracing::warn;
use walkdir::WalkDir;

pub fn collect_mp3_files(inputs: &[PathBuf], recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for input in inputs {
        if !input.exists() {
            eprintln!("存在しないパスです: {}", input.display());
            warn!("Path does not exist: {}", input.display());
            continue;
        }

        if input.is_file() {
            if is_mp3(input) {
                files.push(input.clone());
            } else {
                eprintln!("MP3ではないためスキップ: {}", input.display());
                warn!("Unsupported file: {}", input.display());
            }
            continue;
        }

        if input.is_dir() {
            if recursive {
                collect_recursive(input, &mut files);
            } else {
                collect_non_recursive(input, &mut files);
            }
        }
    }

    files.sort();
    files.dedup();
    files
}

fn collect_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && is_mp3(path) {
            files.push(path.to_path_buf());
        }
    }
}

fn collect_non_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_file() && is_mp3(&path) {
                    files.push(path);
                }
            }
        }
        Err(err) => {
            eprintln!("ディレクトリを読めません: {}: {err}", dir.display());
            warn!("Failed to read directory {}: {err}", dir.display());
        }
    }
}

fn is_mp3(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("mp3"))
        .unwrap_or(false)
}
