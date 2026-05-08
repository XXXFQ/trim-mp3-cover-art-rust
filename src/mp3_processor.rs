use std::path::Path;

use anyhow::{Context, Result};
use id3::{
    frame::{Picture, PictureType},
    Error, ErrorKind, Tag, TagLike, Version,
};
use tracing::debug;

use crate::image_crop::{center_crop, encode_jpeg};

pub enum ProcessResult {
    Processed,
    NoCoverArt,
}

pub fn process_mp3_file(path: &Path, target_aspect_ratio: f32) -> Result<ProcessResult> {
    let mut tag = read_or_create_tag(path)?;

    let picture = match tag.pictures().next().cloned() {
        Some(picture) => picture,
        None => return Ok(ProcessResult::NoCoverArt),
    };

    debug!(
        "Original picture: mime={}, bytes={}",
        picture.mime_type,
        picture.data.len()
    );

    let image = image::load_from_memory(&picture.data)
        .context("埋め込み画像の読み込みに失敗しました")?;

    let cropped = center_crop(image, target_aspect_ratio);
    let jpeg_data = encode_jpeg(cropped).context("JPEGエンコードに失敗しました")?;

    tag.remove_all_pictures();

    tag.add_frame(Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: PictureType::CoverFront,
        description: String::new(),
        data: jpeg_data,
    });

    tag.write_to_path(path, Version::Id3v23)
        .context("ID3タグの保存に失敗しました")?;

    Ok(ProcessResult::Processed)
}

fn read_or_create_tag(path: &Path) -> Result<Tag> {
    match Tag::read_from_path(path) {
        Ok(tag) => Ok(tag),
        Err(Error {
            kind: ErrorKind::NoTag,
            ..
        }) => Ok(Tag::new()),
        Err(err) => Err(err).context("ID3タグの読み込みに失敗しました"),
    }
}
