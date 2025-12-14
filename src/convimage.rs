
use image::{DynamicImage, ImageReader, Rgba};
use palette::{FromColor, IntoColor, Oklaba, Srgba};
use std::path::PathBuf;
use rand::prelude::*;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use lookup::LookupTable;

use crate::lookup;

/// Convertable image.
pub struct ConvImage<'a> {
    /// Name taken from the filename. The resulting image is saved under this name.
    name: String,
    image: Vec<Vec<Oklaba>>,
    /// Noise multiplier. Added to each color during lookup.
    noise: Option<f32>,
    lut: &'a LookupTable,
    converted_image: Vec<Vec<Oklaba>>
}

impl <'a> ConvImage <'a> {
    pub fn new(image_path: PathBuf, lut: &'a LookupTable, noise: Option<f32>) -> Self {
        let name = image_path.file_stem().unwrap().to_str().unwrap().to_string();
        let image = ImageReader::open(image_path).unwrap().decode().unwrap().to_rgba32f();
        let mut image_vec = vec![vec![Oklaba::new(0f32, 0f32, 0f32, 0f32); image.width() as usize]; image.height() as usize];
        let result_vec = image_vec.clone();

        for x in 0..image.width() {
            for y in 0..image.height() {
                let pixel = image.get_pixel(x, y);
                let color: Oklaba = palette::Srgba::from(pixel.0).into_color();
                image_vec[y as usize][x as usize] = color;
            }
        }

        ConvImage {
            name: name,
            image: image_vec,
            noise: noise,
            lut: &lut,
            converted_image: result_vec
        }
    }
    /// Converts the image into a quantized image.
    /// 
    /// Applies the color quantization to the image and stores it.
    /// 
    /// Color is handed over to the LUT. Noise is applied here.
    pub fn convert(mut self) -> Self {
        for x in 0..self.image.len() {
            for y in 0..self.image[x].len() {
                let color = self.image[x][y];
                let mut rng = SmallRng::seed_from_u64((x.pow(2) as f32 + y.pow(3) as f32 + color.l * 10000f32 + color.a * 10000f32 + color.b * 10000f32).abs() as u64);
                self.converted_image[x][y] = self.lut.lookup(color + 
                    Oklaba::new(rng.gen_range(-1.0..=1.0) * self.noise.unwrap_or(0f32), 
                                rng.gen_range(-1.0..=1.0) * self.noise.unwrap_or(0f32), 
                                rng.gen_range(-1.0..=1.0) * self.noise.unwrap_or(0f32), 0f32));
            }
        }

        self
    }

    /// Saves the resulting image as a png.
    /// 
    /// This reduces the final color precision to an u8.
    pub fn save(&self, save_path: &PathBuf) {
        let y_res = self.converted_image.len();
        let x_res = self.converted_image[0].len();
        let mut result_image = DynamicImage::new(x_res as u32, y_res as u32, image::ColorType::Rgba32F).into_rgba32f();

        for y in 0..y_res  {
            for x in 0..x_res {
                let color = self.converted_image[y][x];
                let color_rgb: Srgba<f32> = Srgba::from_color(color);
                let pixel = Rgba::from([color_rgb.red, color_rgb.green, color_rgb.blue, color_rgb.alpha]);
                result_image.put_pixel(x as u32, y as u32, pixel)
            }
        }

        let mut current_save_path = save_path.clone();
        current_save_path.push(self.name.clone());
        current_save_path.set_extension("png");
        let result_image = DynamicImage::from(result_image);
        result_image.into_rgba16().save(current_save_path).unwrap();
    }
}