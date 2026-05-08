mod cli;
mod file_finder;
mod image_crop;
mod logger;
mod mp3_processor;

use anyhow::Result;
use clap::Parser;
use tracing::{info, warn};

use crate::cli::Args;
use crate::file_finder::collect_mp3_files;
use crate::logger::init_debug_logger;
use crate::mp3_processor::{process_mp3_file, ProcessResult};

fn main() -> Result<()> {
    init_debug_logger();

    let args = Args::parse();

    let mp3_files = collect_mp3_files(&args.inputs, args.recursive);

    if mp3_files.is_empty() {
        eprintln!("MP3ファイルが見つかりませんでした。");
        return Ok(());
    }

    for path in mp3_files {
        match process_mp3_file(&path, args.aspect_ratio) {
            Ok(ProcessResult::Processed) => {
                println!("Processed: {}", path.display());
                info!("Processed: {}", path.display());
            }
            Ok(ProcessResult::NoCoverArt) => {
                println!("No cover art: {}", path.display());
            }
            Err(err) => {
                eprintln!("Failed: {}: {err:#}", path.display());
                warn!("Failed: {}: {err:#}", path.display());
            }
        }
    }

    Ok(())
}
