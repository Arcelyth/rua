#![allow(dead_code)]

use anyhow::{Context, Result};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::error::Error;

// rua format
// width height frame
// frame_index, pos_x, pos_y, char, color
pub struct RuaSprite {
    width: u32,
    height: u32,
    frame_num: u32,
    frames: Vec<Option<(char, (u8, u8, u8))>>,
    fps: f64,
    colorful: bool,
}

impl RuaSprite {
    pub fn new(width: u32, height: u32, frame_num: u32, fps: f64, colorful: bool) -> Self {
        let frames: Vec<Option<(char, (u8, u8, u8))>> =
            vec![None; (width * height * frame_num) as usize];
        Self {
            width,
            height,
            frame_num,
            frames,
            fps,
            colorful,
        }
    }

    pub fn from_img(path: String, width: u32, fps: f64) -> Result<Self, Box<dyn Error>> {
        let img = image::open(&path).with_context(|| format!("Failed to open file: {path}"))?;
        let table = get_ascii_table(false);
        let (img_width, img_height) = img.dimensions();

        let ratio = img_height as f32 / img_width as f32;
        let out_height = (ratio * width as f32 * 0.55) as u32;

        let resized_img = img.resize_exact(width, out_height, FilterType::Triangle);
        let gray_img = resized_img.grayscale().into_luma8();

        let mut frames = vec![];
        for y in 0..out_height {
            for x in 0..width {
                let luma_pixel = gray_img.get_pixel(x, y);
                let index = (luma_pixel[0] as f32 / 255.0 * (table.len() - 1) as f32) as usize;
                let c = table.chars().nth(index).unwrap();

                let rgb_pixel = resized_img.get_pixel(x, y);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];
                frames.push(Some((c, (r, g, b))));
            }
        }
        Ok(Self {
            width,
            height: out_height,
            frame_num: 1,
            frames,
            fps,
            colorful: true,
        })
    }

    pub fn to_string(&self, frame: u32) -> String {
        if frame > self.frame_num {
            return "".to_string();
        }
        let mut out = String::new();
        let width = self.width;
        let height = self.height;

        for y in 0..height {
            for x in 0..width {
                let f = (frame - 1) * width * height;
                if let Some(p) = self.frames[(f + (y * width + x)) as usize] {
                    let color = p.1;
                    let colored_char = format!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, p.0);
                    out.push_str(&colored_char);
                }
            }
            out.push_str("\x1b[0m\r\n");
        }

        out
    }
}

pub fn get_ascii_table(detail: bool) -> &'static str {
    if detail {
        r#" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#
    } else {
        " .:-=+*#%"
    }
}
