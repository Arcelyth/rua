#![allow(dead_code)]

use anyhow::{Context, Result};
use image::GenericImageView;
use image::imageops::FilterType;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// rua format
// width height frame
// frame_index, pos_x, pos_y, char, r, g, b
// frame_index, pos_x, pos_y, char, r, g, b
// ...
#[derive(Debug, PartialEq)]
pub struct RuaSprite {
    width: u32,
    height: u32,
    frame_num: u32,
    current_frame: u32,
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
            current_frame: 0,
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
            current_frame: 0,
            frames,
            fps,
            colorful: true,
        })
    }

    pub fn from_rua(path: String, fps: f64, colorful: bool) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut header = String::new();
        reader.read_line(&mut header)?;
        let meta_info: Vec<&str> = header.split_whitespace().collect();
        let width = meta_info.get(0).unwrap().parse::<u32>().unwrap();
        let height = meta_info.get(1).unwrap().parse::<u32>().unwrap();
        let frame_num = meta_info.get(2).unwrap().parse::<u32>().unwrap();
        let mut frames = vec![None; (width * height * frame_num) as usize];
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let pixel: Vec<&str> = line.split_whitespace().collect();
            let f_idx = pixel.get(0).unwrap().parse::<u32>().unwrap();
            let pos_x = pixel.get(1).unwrap().parse::<u32>().unwrap();
            let pos_y = pixel.get(2).unwrap().parse::<u32>().unwrap();
            let ch = pixel.get(3).unwrap().parse::<char>().unwrap();
            let r = pixel.get(4).unwrap().parse::<u8>().unwrap();
            let g = pixel.get(5).unwrap().parse::<u8>().unwrap();
            let b = pixel.get(6).unwrap().parse::<u8>().unwrap();

            frames[((f_idx * width * height) + (width * pos_y) + pos_x) as usize] =
                Some((ch, (r, g, b)));
        }
        Ok(Self {
            width,
            height,
            frame_num: frame_num,
            current_frame: 0,
            frames,
            fps,
            colorful,
        })
    }

    pub fn output_rua(&self, path: String) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{} {} {}", self.width, self.height, self.frame_num)?;

        let pixels_per_frame = (self.width * self.height) as usize;

        for (i, pixel_data) in self.frames.iter().enumerate() {
            if let Some((ch, (r, g, b))) = pixel_data {
                let frame_index = i / pixels_per_frame;
                let local_index = i % pixels_per_frame;

                let pos_y = local_index / (self.width as usize);
                let pos_x = local_index % (self.width as usize);

                writeln!(
                    writer,
                    "{} {} {} {} {} {} {}",
                    frame_index, pos_x, pos_y, ch, r, g, b
                )?;
            }
        }
        writer.flush()?;
        Ok(())
    }

    pub fn get_current_frame(&self) -> &[Option<(char, (u8, u8, u8))>] {
        let size = self.width * self.height;
        let start = size * self.current_frame;
        &self.frames[start as usize..(start + size) as usize]
    }

    pub fn next(&mut self) {
        self.current_frame += 1;
    }

    pub fn insert_frame(&mut self, f: &[Option<(char, (u8, u8, u8))>]) -> bool {
        if f.len() != (self.width * self.height) as usize {
            return false;
        }

        self.frames.extend_from_slice(f);

        self.frame_num += 1;
        true
    }

    pub fn insert_frame_at(&mut self, f: &[Option<(char, (u8, u8, u8))>], pos: u32) -> bool {
        if f.len() != (self.width * self.height) as usize || pos > self.frame_num {
            return false;
        }

        let start_idx = (pos * self.width * self.height) as usize;

        self.frames.splice(start_idx..start_idx, f.iter().cloned());

        self.frame_num += 1;
        true
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
                    let colored_char =
                        format!("\x1b[38;2;{};{};{}m{}", color.0, color.1, color.2, p.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rua() {
        let sprite = RuaSprite::from_rua("./test_file/test.rua".to_string(), 10., true).unwrap();

        let res = RuaSprite {
            width: 10,
            height: 1,
            frame_num: 1,
            current_frame: 0,
            frames: vec![
                Some(('*', (255, 0, 0))),
                Some(('*', (255, 0, 0))),
                Some(('*', (255, 0, 0))),
                Some(('*', (0, 255, 0))),
                Some(('*', (0, 255, 0))),
                Some(('*', (0, 255, 0))),
                None,
                None,
                None,
                None,
            ],
            fps: 10.,
            colorful: true,
        };
        assert_eq!(res, sprite);
    }
}
