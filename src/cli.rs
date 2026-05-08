use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "trim-mp3-cover-art")]
#[command(about = "Trim embedded MP3 cover art to a target aspect ratio")]
pub struct Args {
    /// Input MP3 files or directories containing MP3 files
    #[arg(required = true)]
    pub inputs: Vec<PathBuf>,

    /// Target aspect ratio: width / height
    #[arg(short, long, default_value_t = 1.0)]
    pub aspect_ratio: f32,

    /// Search directories recursively
    #[arg(short, long)]
    pub recursive: bool,
}
