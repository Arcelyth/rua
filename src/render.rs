use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType; 

pub trait Renderable {
    fn to_ascii(&self) -> String; 
    fn to_ascii_colorful(&self) -> String; 
}

#[allow(dead_code)]
pub struct RuaImage {
    pub image: DynamicImage,
    pub width: u32,
    pub detail: bool, 
    pub color: bool,
}

impl RuaImage {

    pub fn from_path(path: String, width: u32, detail: bool, color: bool) -> Self {
        let img = image::open(&path).expect("Failed to open file");
        Self {
            image: img,
            width, detail, color
        }
    }

    pub fn get_ascii_table(detail: bool) -> &'static str {
        if detail {
            r#" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"#
        } else {
            " .:-=+*#%"
        }
    }
}

impl Renderable for RuaImage {
    fn to_ascii(&self) -> String {
        let table = Self::get_ascii_table(self.detail);
        let (img_width, img_height) = self.image.dimensions();

        let ratio = img_height as f32 / img_width as f32;
        let out_height = (ratio * self.width as f32 * 0.55) as u32;
        
        let resized_img = self.image.resize_exact(self.width, out_height, FilterType::Triangle);
        let gray_img = resized_img.grayscale().into_luma8();

        let mut ascii = String::new();
        for (i, pixel) in gray_img.pixels().enumerate() {
            let index = (pixel[0] as f32 / 255.0 * (table.len() - 1) as f32) as usize;
            let c = table.chars().nth(index).unwrap();
            ascii.push(c);

            if (i as u32 + 1) % self.width == 0 {
                ascii.push('\n');
            }
        }
       ascii 
    }

    fn to_ascii_colorful(&self) -> String {
        let table = Self::get_ascii_table(self.detail);
        let (img_width, img_height) = self.image.dimensions();

        let ratio = img_height as f32 / img_width as f32;
        let out_height = (ratio * self.width as f32 * 0.55) as u32;
        
        let resized_img = self.image.resize_exact(self.width, out_height, FilterType::Triangle);
        let gray_img = resized_img.grayscale().into_luma8();

        let mut ascii = String::new();
        
        for y in 0..out_height {
            for x in 0..self.width {
                let luma_pixel = gray_img.get_pixel(x, y);
                let index = (luma_pixel[0] as f32 / 255.0 * (table.len() - 1) as f32) as usize;
                let c = table.chars().nth(index).unwrap();

                let rgb_pixel = resized_img.get_pixel(x, y);
                let r = rgb_pixel[0];
                let g = rgb_pixel[1];
                let b = rgb_pixel[2];

                let colored_char = format!("\x1b[38;2;{};{};{}m{}", r, g, b, c);
                ascii.push_str(&colored_char);
            }
            ascii.push_str("\x1b[0m\n");
        }
        
        ascii
    }
}



