use clap::Parser;
use crate::render::*;

#[derive(Parser, Debug)]
struct Args {
    path: String,

    #[arg(short, long, default_value = "120")]
    width: u32,

    #[arg(short, long, default_value = "false")]
    detail: bool,
    
    #[arg(short, long, default_value = "false")]
    color: bool,
}

pub struct CLI {}

impl CLI {
    pub fn run() {
        let args = Args::parse();
        let img = RuaImage::from_path(args.path, args.width, args.detail, args.color);
        let output = if args.color {
            img.to_ascii_colorful()
        } else {
            img.to_ascii()
        };
        println!("{}", output);
    }
}
