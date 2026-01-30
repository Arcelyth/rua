use clap::Parser;
use image::imageops::FilterType; 
use image::GenericImageView;

#[derive(Parser, Debug)]
struct Args {
    path: String,

    #[arg(short, long, default_value = "120")]
    width: u32,

    #[arg(short, long, default_value = "false")]
    detail: bool,
}

fn main() {
    let ascii_simple_table = " .:-=+*#%";
    let ascii_detail_table = r#" .\'`^"\\,:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@"#;

    let args = Args::parse();
    let out_width = args.width;
    let table = if args.detail { ascii_detail_table } else { ascii_simple_table } ;

    let img = image::open(&args.path).expect("Failed to open file");
    let (img_width, img_height) = img.dimensions();

    let ratio = img_height as f32 / img_width as f32;
    let out_height = (ratio * out_width as f32 * 0.55) as u32;
    
    let resized_img = img.resize_exact(out_width, out_height, FilterType::Nearest);
    let gray_img = resized_img.grayscale().into_luma8();

    let mut ascii_art = String::new();
    for (i, pixel) in gray_img.pixels().enumerate() {
        let index = (pixel[0] as f32 / 255.0 * (table.len() - 1) as f32) as usize;
        let c = table.chars().nth(index).unwrap();
        ascii_art.push(c);

        if (i as u32 + 1) % out_width == 0 {
            ascii_art.push('\n');
        }
    }

    print!("{}", ascii_art);
}
