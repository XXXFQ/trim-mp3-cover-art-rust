use std::io::Cursor;

use anyhow::Result;
use image::{DynamicImage, GenericImageView, ImageFormat};

pub fn center_crop(img: DynamicImage, target_aspect_ratio: f32) -> DynamicImage {
    let (width, height) = img.dimensions();
    let current_aspect_ratio = width as f32 / height as f32;

    if current_aspect_ratio > target_aspect_ratio {
        let new_width = (target_aspect_ratio * height as f32).round() as u32;
        let left = (width - new_width) / 2;

        img.crop_imm(left, 0, new_width, height)
    } else {
        let new_height = (width as f32 / target_aspect_ratio).round() as u32;
        let top = (height - new_height) / 2;

        img.crop_imm(0, top, width, new_height)
    }
}

pub fn encode_jpeg(img: DynamicImage) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());

    // PNG/RGBA系でもJPEG化できるようにRGBへ変換
    let rgb = img.to_rgb8();
    DynamicImage::ImageRgb8(rgb).write_to(&mut buffer, ImageFormat::Jpeg)?;

    Ok(buffer.into_inner())
}
