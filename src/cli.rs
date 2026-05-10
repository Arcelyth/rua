use crate::render::*;
use crate::sprite::*;
use clap::Parser;
use std::error::Error;

use crate::sp::*;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rua ASCII Renderer")]
struct Args {
    path: String,

    #[arg(short, long, default_value = "120")]
    width: u32,

    #[arg(short, long, default_value = "false")]
    detail: bool,

    #[arg(short, long, default_value = "false")]
    color: bool,

    #[arg(short, long, default_value = "false")]
    sprite: bool,

    // Sprite attributes
    #[arg(long, default_value = "1", requires = "sprite")]
    frames: u32,

    #[arg(long, default_value = "0", requires = "sprite")]
    fw: u32,

    #[arg(long, default_value = "0", requires = "sprite")]
    fh: u32,

    #[arg(long, default_value = "10", requires = "sprite")]
    fps: u32,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long)]
    rua: bool,
}

pub struct CLI {}

impl CLI {
    pub fn run() {
        let args = Args::parse();

        let result = if args.sprite {
            Self::handle_sprite(&args)
        } else {
            //            Self::handle_image(&args)
            if args.rua {
                Self::handle_rua(&args)
            } else {
                Self::handle_image(&args)
            }
        };

        if let Err(e) = result {
            eprintln!("\x1b[31m[Error]\x1b[0m Something went wrong!");
            eprintln!("{:?}", e);
        }
    }

    fn handle_image(args: &Args) -> Result<(), Box<dyn Error>> {
        let s = RuaSprite::from_img(args.path.clone(), args.width, 10.)?;
        println!("{}", s.to_string(1));
        if let Some(n) = &args.output {
            if let Err(_) = s.output_rua(n.to_string()) {
                println!("Failed to output.");
            }
        }
        Ok(())
    }

    fn handle_rua(args: &Args) -> Result<(), Box<dyn Error>> {
        let s = RuaSprite::from_rua(args.path.clone(), 10., true)?;
        println!("{}", s.to_string(1));
        if let Some(n) = &args.output {
            if let Err(_) = s.output_rua(n.to_string()) {
                println!("Failed to output.");
            }
        }
        Ok(())
    }


    fn handle_sprite(args: &Args) -> Result<(), Box<dyn Error>> {
        let spr = Sprite::from_path(
            args.path.clone(),
            args.frames,
            args.fw,
            args.fh,
            args.width,
            args.detail,
            args.color,
            args.fps,
            None,
        )?;

        spr.play()?;

        Ok(())
    }
}
