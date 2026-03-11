use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use image::DynamicImage;
use std::io::{self, Write};
use std::time::Duration;

use crate::render::*;

#[allow(dead_code)]
pub struct Sprite {
    pub sheet: DynamicImage,
    pub num_frames: u32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub target_width: u32,
    pub detail: bool,
    pub color: bool,
    pub frame_duration: Duration,
}

impl Sprite {
    pub fn from_path(
        path: String,
        num_frames: u32,
        frame_width: u32,
        frame_height: u32,
        target_width: u32,
        detail: bool,
        color: bool,
        fps: u32,
    ) -> Result<Self> {
        let img =
            image::open(&path).with_context(|| format!("Failed to open sprite sheet: {path}"))?;

        let frame_duration = Duration::from_millis((1000.0 / fps as f32) as u64);

        Ok(Self {
            sheet: img,
            num_frames,
            frame_width,
            frame_height,
            target_width,
            detail,
            color,
            frame_duration,
        })
    }

    fn get_frame(&self, index: u32) -> RuaImage {
        let sheet_width = self.sheet.width();
        let cols = sheet_width / self.frame_width;

        let row = index / cols;
        let col = index % cols;

        let start_x = col * self.frame_width;
        let start_y = row * self.frame_height;

        let frame_img = self
            .sheet
            .crop_imm(start_x, start_y, self.frame_width, self.frame_height);

        RuaImage {
            image: frame_img,
            width: self.target_width,
            detail: self.detail,
            color: self.color,
        }
    }

    pub fn play(&self) -> anyhow::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(
            stdout,
            crossterm::cursor::Hide,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;

        let mut current_frame = 0;

        let result = loop {
            let frame = self.get_frame(current_frame);
            let ascii_frame = if self.color {
                frame.to_ascii_colorful()
            } else {
                frame.to_ascii()
            };

            print!("\x1b[H{}", ascii_frame);
            stdout.flush()?;

            if event::poll(self.frame_duration)? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                        _ => {}
                    }
                }
            }

            current_frame = (current_frame + 1) % self.num_frames;
            print!("Animation playing... Press 'q' or 'Esc' to exit.");
            stdout.flush()?;
        };

        execute!(stdout, crossterm::cursor::Show)?;
        disable_raw_mode()?;

        println!();

        result
    }
}
