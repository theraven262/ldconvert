//! # LUT-Diffuse Converter
//! 
//! LUT-Diffuse Converter creates an error-diffused LUT from a color palette and uses it to convert images into that palette.
//! 
//! The approach uses "minimized average error" error-diffusion algorithm on the LUT, expanded to diffuse over all three channels. Color calculations are performed in the Oklab color space.
//! 
//! Source palette is a png file, colors can repeat, but increase calculation time.
//! Alpha channel is unaffected by the conversion. Palette index 0 is skipped.
use clap::Parser;
use convimage::ConvImage;
use lookup::{LookupTable, SaveableLookupTable};
use std::path::PathBuf;
use std::thread;
extern crate savefile;
use savefile::prelude::*;

mod lookup;
mod convimage;

#[macro_use]
extern crate savefile_derive;

#[derive(Parser, Debug)]
#[command(version, about = "LUT-Diffuse Converter", long_about = "LUT-Diffuse Converter. Makes a diffused lookup table from a color palette. Converts images into the desired color palette.")]
struct Args {
    /// Palette image path.
    /// 
    /// Ensure one pixel per color entry.
    /// 
    /// Colors can repeat, but redundancy increases LUT generation time.
    #[arg(short, long)]
    palette_path: Option<String>,
    /// Resulting LUT save path.
    /// 
    /// If used, the resulting LUT is saved in the provided directory.
    /// 
    /// Also used to store the optional LUT slices.
    #[arg(short, long)]
    save_path: Option<String>,
    /// LUT load path.
    /// 
    /// The location of a bin file which stores a LUT.
    #[arg(short, long)]
    load_path: Option<String>,
    /// LUT precision, number of discrete steps for each of the RGB channels.
    /// 
    /// Use 255 for full u8 png precision.
    #[arg(short, long, default_value = "16")]
    resolution: u32,
    /// Whether to save images of the resulting LUT.
    /// 
    /// These slice images are saved in the LUT save path.
    #[arg(long, default_value = "false")]
    save_slices: bool,
    /// Path to a directory containing the images to convert.
    #[arg(short, long)]
    convert_source_path: Option<String>,
    /// Path to save the resulting images in.
    /// 
    /// Created if not found.
    /// 
    /// Defaults to the "converted" directory in the source path.
    #[arg(short = 'd', long)]
    convert_destination_path: Option<String>,
    /// Additional noise to apply during conversion.
    /// 
    /// If used, keep in the range of 0.005 to 0.025 for best results
    #[arg(short, long, default_value = "0")]
    noise: Option<f32>,

    #[arg(long, hide = true)]
    markdown_help: bool,
}
fn main() {
    let args = Args::parse();

    // Generates help markdown
    if args.markdown_help {
        clap_markdown::print_help_markdown::<Args>();
    }

    let lut: LookupTable;

    match args.load_path {
        Some(load_path) => {
            let load_path = PathBuf::from(load_path);
            if (load_path.is_file() == false) || (load_path.extension().unwrap().to_str().unwrap() != "bin") {
                panic!("Load path does not point to a bin file.")
            }
            println!("Loading LUT...");
            lut = LookupTable::from(load_file::<SaveableLookupTable, _>(load_path, 1).unwrap());
        }
        None =>  match args.palette_path {
            Some(palette_path) => {
                let palette_path = PathBuf::from(palette_path);
                if palette_path.is_file() == false {
                    panic!("Palette path is not a file.")
                }
                lut = LookupTable::new(PathBuf::from(palette_path), args.resolution as usize).populate().discretize();
            }
            None => {
                panic!("No palette or load path provided.");
            }
        },
    }

    match args.save_path {
        Some(save_path) => {
            let save_path = PathBuf::from(save_path);
            if save_path.exists() == false {
                std::fs::create_dir(&save_path).unwrap();
                println!("Created save path.")
            }
            lut.save_lut(PathBuf::from(&save_path));
            if args.save_slices {
                println!("Saving slices...");
                lut.save_slices(PathBuf::from(&save_path));
            }
        }
        None => {}
    }

    match args.convert_source_path {
        Some(source_path) => {
            let source_path = PathBuf::from(source_path);
            if source_path.is_dir() == false {
                panic!("Source path is not a directory.")
            }
            let destination_path = match args.convert_destination_path {
                Some(destination_path) => PathBuf::from(destination_path),
                None => {
                    let mut destination_path = source_path.clone();
                    destination_path.push("converted");
                    if destination_path.is_dir() == false {
                        std::fs::create_dir(&destination_path).unwrap();
                    }
                    destination_path
                },
            };
            thread::scope(|s| {
                for image_file in std::fs::read_dir(source_path).unwrap() {
                    if image_file.as_ref().unwrap().path().is_file() {
                        s.spawn(|| {
                            let image_file = image_file.unwrap();
                            println!("Converting {}", image_file.file_name().to_str().unwrap());
                            ConvImage::new(image_file.path(), &lut, args.noise).convert().save(&destination_path);
                        });
                    }
                }
            });
        }
        None => {}
    }
}