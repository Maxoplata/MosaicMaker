/**
 * MosaicMaker.rs
 *
 * Creates a mosaic image png
 * usage: cargo build && ./MosaicMaker TILE_SIZE "input url/filepath" "output png file"
 * example: cargo build MosaicMaker.go && ./MosaicMaker 20 "https://raw.githubusercontent.com/Maxoplata/MosaicMaker/main/_readmeAssets/sampleInput.jpg" "./mosaic.png"
 * example: cargo build MosaicMaker.go && ./MosaicMaker 20 "./sampleInput.jpg" "./mosaic.png"
 *
 * @author Maxamilian Demian
 * @link https://www.maxodev.org
 * @link https://github.com/Maxoplata/MosaicMaker
 */
use std::{env, path, process};
use image::{DynamicImage, GenericImageView, ImageBuffer, imageops};
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    // if we have arguments passed to the script
    if args.len() != 4 {
        println!("Invalid argument count");
        process::exit(1);
    }

    // vars
    let tile_size: u32 = args[1].trim().parse().expect("TILE_SIZE expects a numeric value");
    let input_file = &args[2];
    let output_file = &args[3];

    // validate the tile size
    if tile_size < 2 {
        println!("Invalid tile size (minimum 2)");
        process::exit(1);
    }

    // validate input file
    let img_orig = if path::Path::new(input_file).exists() {
        image::open(input_file).unwrap()
    } else {
        let img_from_url = match reqwest::blocking::get(input_file) {
            Ok(res) => {
                if res.status() != 200 {
                    println!("File does not exist");
                    process::exit(1);
                }

                res
            },
            Err(_) => {
                println!("Unknown file error");
                process::exit(1);
            },
        };

        let image_from_url_bytes = match img_from_url.bytes() {
            Ok(res) => {
                res
            },
            Err(_) => {
                println!("Unknown file error");
                process::exit(1);
            },
        };

        let image_from_url_bytes_loaded = match image::load_from_memory(&image_from_url_bytes) {
            Ok(res) => {
                res
            },
            Err(_) => {
                println!("Unknown file error");
                process::exit(1);
            },
        };

        image_from_url_bytes_loaded
    };

    // create image tile
    let img_tile = img_orig.thumbnail(tile_size, tile_size).to_rgba8();

    // get width/height of image
    let (width_orig, height_orig) = img_orig.dimensions();

    // create new image
    let mut img_new = DynamicImage::ImageRgba8(ImageBuffer::new(width_orig * tile_size, height_orig * tile_size));

    // iterate through original image pixels
    for x in 0..width_orig {
        for y in 0..height_orig {
            // copy image tile to new image
            imageops::overlay(&mut img_new, &img_tile, x * tile_size, y * tile_size);

            // get pixel color from original image
            let pixel = img_orig.get_pixel(x, y);

            // create color tile
            let img_color = DynamicImage::ImageRgba8(ImageBuffer::from_fn(tile_size, tile_size, |_x, _y| {
                image::Rgba([pixel.0[0] , pixel.0[1], pixel.0[2], 127])
            }));

            // copy color tile to new image
            imageops::overlay(&mut img_new, &img_color, x * tile_size, y * tile_size);
        }
    }

    // save image to file
    img_new.save(output_file).unwrap();
}
