use crate::render::*;
use crate::sprite::*;
use clap::Parser;

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
}

pub struct CLI {}

impl CLI {
    pub fn run() {
        let args = Args::parse();

        let result = if args.sprite {
            Self::handle_sprite(&args)
        } else {
            Self::handle_image(&args)
        };

        if let Err(e) = result {
            eprintln!("\x1b[31m[Error]\x1b[0m Something went wrong!");
            eprintln!("{:?}", e);
        }
    }

    fn handle_image(args: &Args) -> anyhow::Result<()> {
        let img = RuaImage::from_path(args.path.clone(), args.width, args.detail, args.color)?;

        let output = if args.color {
            img.to_ascii_colorful()
        } else {
            img.to_ascii()
        };

        println!("{}", output);
        Ok(())
    }

    fn handle_sprite(args: &Args) -> anyhow::Result<()> {
        let spr = Sprite::from_path(
            args.path.clone(),
            args.frames,
            args.fw,
            args.fh,
            args.width,
            args.detail,
            args.color,
            args.fps,
        )?;


        spr.play()?;

        Ok(())
    }
}
