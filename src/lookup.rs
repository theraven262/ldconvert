
use image::{DynamicImage, GenericImage, ImageReader, Rgba};
use palette::color_difference::EuclideanDistance;
use palette::{FromColor, IntoColor, Oklab, Oklaba, Srgb, WithAlpha};
use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
extern crate savefile;
use savefile::prelude::*;

/// Struct used to save Oklab values using Savefile crate.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Savefile)]
struct SaveableOklab {
    l: f32,
    a: f32,
    b: f32
}

/// Generates a LUT based on a palette, and does color quantization for convertable images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookupTable {
    /// Used for the naming of the save file.
    name: String,
    values: Vec<Vec<Vec<Oklab>>>,
    /// LUT resolution.
    /// 
    /// The number of steps that each color channel is broken into.
    /// 
    /// For the png format, 255 covers all the possible colors.
    resolution: usize,
    palette: HashMap<u32, Oklab>,
    /// Diffusion kernel.
    /// Stores the weights used to distribute the error value over nearby colors when quantizing the LUT.
    kernel: Vec<KernelEntry>
}

/// Same as a LUT, but can be saved using Savefile.
#[derive(Debug, Clone, Serialize, Deserialize, Savefile)]
pub struct SaveableLookupTable {
    name: String,
    values: Vec<Vec<Vec<SaveableOklab>>>,
    resolution: usize,
    palette: HashMap<u32, SaveableOklab>,
    kernel: Vec<KernelEntry>
}

/// Stores the relative coordinates of the color that a part of the error is diffused to.
/// 
/// Stores a single weight which describes the amount of error to be diffused to the target color.
#[derive(Debug, Clone, Serialize, Deserialize, Savefile)]
struct KernelEntry {
    /// Number of steps to the target color in the red direction.
    x_offset: i32,
    /// Number of steps to the target color in the green direction.
    y_offset: i32,
    /// Number of steps to the target color in the blue direction.
    z_offset: i32,
    /// Amount of error that this kernel adds to the target color.
    scale: f32
}

impl KernelEntry {
    fn new(x: i32, y: i32, z: i32, scale: f32) -> KernelEntry {
        KernelEntry{
            x_offset: x,
            y_offset: y,
            z_offset: z,
            scale: scale
        }
    }
}

impl SaveableOklab {
    fn new(lab_color: Oklab) -> SaveableOklab {
        SaveableOklab { l: lab_color.l, a: lab_color.a, b: lab_color.b }
    }
}

impl SaveableOklab {
    fn inner(self) -> Oklab {
        Oklab::from_components((self.l, self.a, self.b))
    }
}

impl From<Oklab> for SaveableOklab {
    fn from(color: Oklab) -> Self {
        SaveableOklab::new(color)
    }
}

/// Storage retrieval conversion for LUT.
impl From<SaveableLookupTable> for LookupTable {
    fn from(lut: SaveableLookupTable) -> LookupTable {
        let resolution = lut.resolution;
        let mut values = vec![vec![vec![Oklab::new(0f32, 0f32, 0f32); resolution]; resolution]; resolution];
        let mut palette: HashMap<u32, Oklab> = HashMap::new();

        for b in 0..resolution {
            for g in 0..resolution {
                for r in 0..resolution {
                    let lab_color = lut.values[b][g][r].inner();
                    values[b][g][r] = lab_color;
                }
            }
        }

        for color in lut.palette {
            palette.insert(color.0, color.1.inner());
        }
        
        LookupTable {
            name: lut.name,
            values: values,
            resolution: lut.resolution,
            palette: palette,
            kernel: lut.kernel
        }
    }
}

/// Storage conversion for LUT.
impl Into<SaveableLookupTable> for &LookupTable {
    fn into(self) -> SaveableLookupTable {
        let resolution = self.resolution;
        let mut values = vec![vec![vec![SaveableOklab::new(Oklab::new(0f32, 0f32, 0f32)); resolution]; resolution]; resolution];
        let mut palette: HashMap<u32, SaveableOklab> = HashMap::new();

        for b in 0..resolution {
            for g in 0..resolution {
                for r in 0..resolution {
                    let lab_color = SaveableOklab::new(self.values[b][g][r]);
                    values[b][g][r] = lab_color;
                }
            }
        }

        for color in &self.palette {
            palette.insert(*color.0, SaveableOklab::new(*color.1));
        }

        SaveableLookupTable {
            name: self.name.clone(),
            values: values,
            resolution: self.resolution,
            palette: palette,
            kernel: self.kernel.clone()
        }
    }
}

impl LookupTable {

    /// Initializes fields and fills the kernel vector.
    /// 
    /// Kernel weights are distributed according to the Minimized Average Error algorithm, extended to the third dimension.
    /// This divides the error into 108 parts.
    pub fn new(palette_path: PathBuf, resolution: usize) -> LookupTable {

        let mut lut = LookupTable {
            name: palette_path.file_stem().unwrap().to_str().unwrap().to_string(),
            values: vec![vec![vec![Oklab::new(0f32, 0f32, 0f32); resolution]; resolution]; resolution],
            resolution: resolution,
            palette: HashMap::new(),
            kernel: vec![
                KernelEntry::new(1, 0, 0, 7.0), KernelEntry::new(2, 0, 0, 5.0),
                KernelEntry::new(-2, 1, 0, 3.0), KernelEntry::new(-1, 1, 0, 5.0), KernelEntry::new(0, 1, 0, 7.0), KernelEntry::new(1, 1, 0, 5.0), KernelEntry::new(2, 1, 0, 3.0),
                KernelEntry::new(-2, 2, 0, 1.0), KernelEntry::new(-1, 2, 0, 3.0), KernelEntry::new(0, 2, 0, 5.0), KernelEntry::new(1, 2, 0, 3.0), KernelEntry::new(2, 2, 0, 1.0),
                
                KernelEntry::new(-2, 0, 1, 3.0), KernelEntry::new(-1, 0, 1, 5.0), KernelEntry::new(0, 0, 1, 7.0), KernelEntry::new(1, 0, 1, 5.0), KernelEntry::new(2, 0, 1, 3.0),
                KernelEntry::new(-2, 1, 1, 1.0), KernelEntry::new(-1, 1, 1, 3.0), KernelEntry::new(0, 1, 1, 5.0), KernelEntry::new(1, 1, 1, 3.0), KernelEntry::new(2, 1, 1, 1.0),
                KernelEntry::new(-1, 2, 1, 1.0), KernelEntry::new(0, 2, 1, 3.0), KernelEntry::new(1, 2, 1, 1.0),
                
                KernelEntry::new(-2, 0, 2, 1.0), KernelEntry::new(-1, 0, 2, 3.0), KernelEntry::new(0, 0, 2, 5.0), KernelEntry::new(1, 0, 2, 3.0), KernelEntry::new(2, 0, 2, 1.0),
                KernelEntry::new(-1, 1, 2, 1.0), KernelEntry::new(0, 1, 2, 3.0), KernelEntry::new(1, 1, 2, 1.0),
                KernelEntry::new(0, 2, 2, 1.0)
            ]
        };

        let palette_image = ImageReader::open(palette_path).unwrap().decode().unwrap().to_rgb32f();
        let mut id = 0u32;

        for y in 0..palette_image.height() {
            for x in 0..palette_image.width() {
                let pixel = palette_image.get_pixel(x, y);
                let color: Oklab = palette::Srgb::from(pixel.0).into_linear().into_color();
                lut.palette.insert(id, color);
                id = id + 1u32;
            }
        }

        lut
    }

    /// Fills the LUT with clean RGB values.
    pub fn populate(mut self) -> Self {
        println!("Populating lookup table...");
        for b in 0..self.resolution {
            for g in 0..self.resolution {
                for r in 0..self.resolution {
                    let rgb_color: Srgb<f32> = Srgb::new(r as f32 / self.resolution as f32, g as f32 / self.resolution as f32, b as f32 / self.resolution as f32);
                    let lab_color = Oklab::from_color(rgb_color);
                    self.values[b][g][r] = lab_color;
                }
            }
        }

        self
    }

    /// Quantizes the LUT with error diffusion.
    pub fn discretize(mut self) -> Self {
        for b in 0..self.resolution {
            println!("Discretizing slice {} out of {}", b, self.resolution);
            for g in 0..self.resolution {
                for r in 0..self.resolution {
                    let color = self.values[b][g][r];
                    let closest_match = LookupTable::discretize_color(&self.palette, &color);
                    let difference = LookupTable::scale(color - closest_match, 1f32 / 108f32);

                    self.values[b][g][r] = closest_match;

                    for i in &self.kernel {
                        let x_offset = r as i32 + i.x_offset;
                        let y_offset = g as i32 + i.y_offset;
                        let z_offset = b as i32 + i.z_offset;

                        if (x_offset < 0i32) || (x_offset > self.resolution as i32 - 1i32) {
                            break
                        }

                        if (y_offset < 0i32) || (y_offset > self.resolution as i32 - 1i32) {
                            break
                        }

                        if (z_offset < 0i32) || (z_offset > self.resolution as i32 - 1i32) {
                            break
                        }

                        self.values[z_offset as usize][y_offset as usize][x_offset as usize] += LookupTable::scale(difference.clone(), i.scale);
                    }
                }
            }
        }

        self
    }

    /// Quantizes a color to the nearest color to it in the palette.
    /// 
    /// Uses Oklab euclidean distance for color difference calculation.
    fn discretize_color(palette: &HashMap<u32, Oklab>, color: &Oklab) -> Oklab {
        let mut latest_distance = 1000f32;
        let mut target = Oklab::new(0f32, 0f32, 0f32);

        for i in 1..palette.len() {
            let distance = Oklab::from_color(*color).distance(Oklab::from_color(palette[&(i as u32)]));
            if distance < latest_distance {
                latest_distance = distance;
                target = palette[&(i as u32)];
            }
        };

        target
    }

    /// Scales the color components.
    /// 
    /// Used for error diffusion, result otherwise nonsensical.
    fn scale(color: Oklab, scale: f32) -> Oklab {
        Oklab::new(color.a * scale, color.b * scale, color.l * scale)
    }

    /// Applies the LUT to a convertible image color.
    /// 
    /// The color is rounded to the nearest valid LUT step, in case of a low resolution LUT.
    pub fn lookup(&self, color: Oklaba) -> Oklaba {
        let rgb: Srgb = color.into_color();

        let mut blue = (rgb.blue * self.resolution as f32).round() as usize;
        if blue > self.resolution - 1 {
            blue = self.resolution - 1;
        }
        let mut green = (rgb.green * self.resolution as f32).round() as usize;
        if green > self.resolution - 1 {
            green = self.resolution - 1;
        }
        let mut red = (rgb.red * self.resolution as f32).round() as usize;
        if red > self.resolution - 1 {
            red = self.resolution - 1;
        }

        Oklaba::from(self.values[blue][green][red]).with_alpha(color.alpha)
    }

    /// Saves slices of the lookup table as images using the provided save path.
    /// 
    /// Slices are used for testing the validity and coverage of the conversion.
    pub fn save_slices(&self, save_path: PathBuf) {
        let mut n = 0i32;
        for slice in &self.values {
            let mut image = DynamicImage::new(self.resolution as u32, self.resolution as u32, image::ColorType::Rgb32F);

            for x in 0..self.resolution {
                for y in 0..self.resolution {
                    let color = slice[x][y];
                    let color_rgb: Srgb<f32> = Srgb::from_color(color);
                    let pixel = Rgba::<u8>::from([(color_rgb.red * 255f32) as u8, (color_rgb.green * 255f32) as u8, (color_rgb.blue * 255f32) as u8, u8::MAX]);
                    image.put_pixel(x as u32, y as u32, pixel)
                }
            }
        let mut current_save_path = save_path.clone();
        current_save_path.push(format!("{}_slice_{}", &self.name, n));
        current_save_path.set_extension("png");
        image.into_rgb8().save(current_save_path).unwrap();
        n = n + 1i32;
        }
    }

    /// Saves the LUT as a binary file using the provided save path.
    pub fn save_lut(&self, save_path: PathBuf) {
        let mut save_path = save_path.clone();
        save_path.push(self.name.to_string() + "_lut.bin");
        let save_lut: SaveableLookupTable = self.into();
        save_file(save_path, 0, &save_lut).unwrap();
    }
}