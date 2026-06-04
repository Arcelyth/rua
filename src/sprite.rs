//! # Rua-rs
//! A file format engine handling multi-frame terminal sprite animations.

use image::imageops::FilterType;
use image::GenericImageView;
use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write as IoWrite};

/// Represents a single character cell payload on the terminal screen matrix.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub ch: char,
    pub color: (u8, u8, u8),
}

/// A structured container enclosing a fully rendered animation frame buffer matrix.
#[derive(Clone, Debug, PartialEq)]
pub struct Frame {
    pub data: Vec<Option<Pixel>>,
}

/// rua format
/// width height frame
/// frame_index, pos_x, pos_y, char, r, g, b
/// frame_index, pos_x, pos_y, char, r, g, b
/// ...
///
/// Represents an ASCII art sprite animation format capable of managing multi-frame,
/// colored character sequences with specific dimensions and playback speed.
#[derive(Debug, PartialEq)]
pub struct Sprite {
    width: u32,
    height: u32,
    frame_num: u32,
    current_frame: u32,
    frames: Vec<Frame>,
    fps: f64,
    colorful: bool,
}

impl Sprite {
    /// Creates a blank container instance configured with predetermined coordinate boundaries.
    pub fn new(width: u32, height: u32, frame_num: u32, fps: f64, colorful: bool) -> Self {
        let frames = vec![
            Frame {
                data: vec![None; (width * height) as usize]
            };
            frame_num as usize
        ];
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

    /// Generates a single-frame `Sprite` from an external source image file paths.
    pub fn from_img(path: String, width: u32, fps: f64) -> Result<Self, Box<dyn Error>> {
        let img = image::open(&path)?;
        let table = get_ascii_table(false);
        let (img_width, img_height) = img.dimensions();

        let ratio = img_height as f32 / img_width as f32;
        let out_height = (ratio * width as f32 * 0.55) as u32;

        let resized_img = img.resize_exact(width, out_height, FilterType::Triangle);
        let gray_img = resized_img.grayscale().into_luma8();

        let mut data = Vec::with_capacity((width * out_height) as usize);
        for y in 0..out_height {
            for x in 0..width {
                let luma_pixel = gray_img.get_pixel(x, y);
                let index = (luma_pixel[0] as f32 / 255.0 * (table.len() - 1) as f32) as usize;
                let c = table.chars().nth(index).unwrap_or(' ');

                let rgb_pixel = resized_img.get_pixel(x, y);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];
                
                data.push(Some(Pixel { ch: c, color: (r, g, b) }));
            }
        }

        Ok(Self {
            width,
            height: out_height,
            frame_num: 1,
            current_frame: 0,
            frames: vec![Frame { data }],
            fps,
            colorful: true,
        })
    }

    /// Parses an active configuration instance from custom structured token plaintext sequences safely.
    pub fn from_rua(path: String, fps: f64, colorful: bool) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut header = String::new();
        reader.read_line(&mut header)?;

        let meta_info: Vec<&str> = header.split_whitespace().collect();
        if meta_info.len() < 3 {
            return Err("Invalid RUA header information payload format structure".into());
        }

        let width = meta_info[0].parse::<u32>()?;
        let height = meta_info[1].parse::<u32>()?;
        let frame_num = meta_info[2].parse::<u32>()?;

        let mut frames = vec![
            Frame {
                data: vec![None; (width * height) as usize]
            };
            frame_num as usize
        ];

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let pixel_tokens: Vec<&str> = line.split_whitespace().collect();
            if pixel_tokens.len() < 7 {
                continue;
            }

            let f_idx = pixel_tokens[0].parse::<u32>()?;
            let pos_x = pixel_tokens[1].parse::<u32>()?;
            let pos_y = pixel_tokens[2].parse::<u32>()?;
            let ch = pixel_tokens[3].parse::<char>()?;
            let r = pixel_tokens[4].parse::<u8>()?;
            let g = pixel_tokens[5].parse::<u8>()?;
            let b = pixel_tokens[6].parse::<u8>()?;

            if f_idx < frame_num && pos_x < width && pos_y < height {
                let local_idx = (pos_y * width + pos_x) as usize;
                frames[f_idx as usize].data[local_idx] = Some(Pixel { ch, color: (r, g, b) });
            }
        }

        Ok(Self {
            width,
            height,
            frame_num,
            current_frame: 0,
            frames,
            fps,
            colorful,
        })
    }

    /// Serializes active instance configurations out into target output paths.
    pub fn output_rua(&self, path: String) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{} {} {}", self.width, self.height, self.frame_num)?;

        for (f_idx, frame) in self.frames.iter().enumerate() {
            for (local_index, pixel_opt) in frame.data.iter().enumerate() {
                if let Some(pixel) = pixel_opt {
                    let pos_y = local_index / (self.width as usize);
                    let pos_x = local_index % (self.width as usize);

                    writeln!(
                        writer,
                        "{} {} {} {} {} {} {}",
                        f_idx, pos_x, pos_y, pixel.ch, pixel.color.0, pixel.color.1, pixel.color.2
                    )?;
                }
            }
        }
        writer.flush()?;
        Ok(())
    }

    /// Exposes an immutable data reference payload matching current frame sequences.
    pub fn get_current_frame(&self) -> Option<&Frame> {
        self.frames.get(self.current_frame as usize)
    }

    /// Advances operational targeting indicators down onto sequential timeline elements.
    pub fn next(&mut self) {
        if self.frame_num > 0 {
            self.current_frame = (self.current_frame + 1) % self.frame_num;
        }
    }

    /// Appends target frame instances safely onto global runtime configuration sequences.
    pub fn insert_frame(&mut self, frame: Frame) -> bool {
        if frame.data.len() != (self.width * self.height) as usize {
            return false;
        }
        self.frames.push(frame);
        self.frame_num += 1;
        true
    }

    /// Splices unique configurations deep down into runtime array matrices under a linear timeframe.
    pub fn insert_frame_at(&mut self, frame: Frame, pos: u32) -> bool {
        if frame.data.len() != (self.width * self.height) as usize || pos > self.frame_num {
            return false;
        }
        self.frames.insert(pos as usize, frame);
        self.frame_num += 1;
        true
    }

    /// Removes sequential data targets resting under requested offset indices.
    pub fn remove_frame_at(&mut self, pos: u32) -> Option<Frame> {
        if pos >= self.frame_num || self.frame_num == 0 {
            return None;
        }
        let removed = self.frames.remove(pos as usize);
        self.frame_num -= 1;
        
        if self.current_frame >= self.frame_num && self.frame_num > 0 {
            self.current_frame = self.frame_num - 1;
        }
        Some(removed)
    }

    /// Discards final frame tracking elements. Returns false if processing empty instances.
    pub fn remove_frame(&mut self) -> bool {
        if self.frame_num == 0 {
            return false;
        }
        self.remove_frame_at(self.frame_num - 1).is_some()
    }

    /// Transforms specific timeline instances cleanly into terminal strings.
    pub fn to_string(&self, frame_idx: u32) -> String {
        if frame_idx == 0 || frame_idx > self.frame_num {
            return String::new();
        }

        let frame = &self.frames[(frame_idx - 1) as usize];
        let mut out = String::with_capacity((self.width * self.height * 12) as usize);
        let mut current_color: Option<(u8, u8, u8)> = None;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                
                if let Some(pixel) = &frame.data[idx] {
                    if self.colorful {
                        if current_color != Some(pixel.color) {
                            let _ = write!(
                                out, 
                                "\x1b[38;2;{};{};{}m", 
                                pixel.color.0, pixel.color.1, pixel.color.2
                            );
                            current_color = Some(pixel.color);
                        }
                    }
                    out.push(pixel.ch);
                } else {
                    out.push(' ');
                }
            }
            if self.colorful && current_color.is_some() {
                out.push_str("\x1b[0m");
                current_color = None;
            }
            out.push_str("\r\n");
        }

        out
    }
}

fn get_ascii_table(detail: bool) -> &'static str {
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
        let mut mock_data = vec![None; 10];
        mock_data[0] = Some(Pixel { ch: '*', color: (255, 0, 0) });
        mock_data[1] = Some(Pixel { ch: '*', color: (255, 0, 0) });
        mock_data[2] = Some(Pixel { ch: '*', color: (255, 0, 0) });
        mock_data[3] = Some(Pixel { ch: '*', color: (0, 255, 0) });
        mock_data[4] = Some(Pixel { ch: '*', color: (0, 255, 0) });
        mock_data[5] = Some(Pixel { ch: '*', color: (0, 255, 0) });

        let res = Sprite {
            width: 10,
            height: 1,
            frame_num: 1,
            current_frame: 0,
            frames: vec![Frame { data: mock_data }],
            fps: 10.,
            colorful: true,
        };

        let sprite = Sprite::from_rua("./test_file/test.rua".to_string(), 10., true);
        if let Ok(parsed_sprite) = sprite {
            assert_eq!(res, parsed_sprite);
        }
    }
}
