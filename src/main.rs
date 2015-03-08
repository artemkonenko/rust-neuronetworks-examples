#![feature(old_io)]
#![feature(old_path)]

extern crate image;
extern crate neuralnetwork;
extern crate num;

use image::GenericImage;
use neuralnetwork::{HopfieldNetwork};
use std::num::Float;
use std::old_io::File;
use std::old_io::fs;
use std::old_path::Path;

fn image_to_pattern(path: &Path) -> Vec<f64> {
    let img = image::open(path).unwrap();

    let mut res : Vec<f64> = Vec::with_capacity((img.height() * img.width()) as usize);
    for y in 0..img.height() {
        for x in 0..img.width() {
            res.push( if img.get_pixel(x, y)[0] > 200 { 1.0 } else { -1.0 } );
        }
    }
    res
}

fn pattern_to_image(pattern: Vec<f64>, result_path: &str) {
    let mut imgbuf = image::ImageBuffer::new(90, 90);
    println!("> Save test result {:?}", result_path);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let res_color = if pattern[(y * 90 + x) as usize] > 0f64 { 255 } else { 0 };
        *pixel = image::Luma([res_color]);
    }

    let ref mut fout = File::create(&Path::new(result_path)).unwrap();
    let _ = image::ImageLuma8(imgbuf).save(fout, image::PNG);
}

// Srly? Or maybe I should found it yet another time?
fn str_concat(dir: &str, file: &str) -> String {
    let mut a = dir.to_string();
    a.push_str(file);
    a.to_string()
}

fn main() {
    let study_dir = "images/BWpack/";
    let test_dir = "images/BWpack_noised/";
    let result_dir = "images/result/";

    let mut study_pictures_src : Vec<Path> = Vec::new();
    match fs::readdir(&Path::new(study_dir)) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(paths) => for path in paths.iter() {
            match path.extension_str() {
                None => println!("! file without extension was ignored."),
                Some(ext) => if ext == "png" {
                    println!("> Found study picture {}", path.display());
                    study_pictures_src.push(path.clone());
                }
            }
        },
    }

    let mut study_patterns : Vec<Vec<f64>> = Vec::with_capacity(study_pictures_src.len());
    for path in study_pictures_src {
        study_patterns.push( image_to_pattern( &path ) );
    }

    let network = HopfieldNetwork::new(study_patterns, Float::signum );

    let mut test_pictures_src : Vec<Path> = Vec::new();
    match fs::readdir(&Path::new(test_dir)) {
        Err(why) => println!("! {:?}", why.kind),
        Ok(paths) => for path in paths.iter() {
            match path.extension_str() {
                None => println!("! file without extension was ignored."),
                Some(ext) => if ext == "png" {
                    println!("> Found test picture {}", path.display());
                    test_pictures_src.push(path.clone());
                }
            }
        },
    }
    for test_image_path in test_pictures_src {
        let input = image_to_pattern( &test_image_path );
        //let mut output;
        let output = network.feed(&input);

        pattern_to_image(output, &str_concat(result_dir, test_image_path.filename_str().unwrap() ) );
    }
}
