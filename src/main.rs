#![feature(old_io)]
#![feature(old_path)]
extern crate neuralnetwork;
extern crate image;
extern crate num;

use std::old_io::File;
use image::GenericImage;
use num::complex::Complex;
use neuralnetwork::Network;

fn image_to_pattern(path: &str) -> Vec<u8> {
    println!("{:?}", path);
    vec![1u8, 2]
}

fn main() {
    let network = Network::new(vec!(2, 3, 1));

    let study_pictures_src = vec!["A.png", "B.png", "C.png"];

    for path in study_pictures_src {
        let _ = image_to_pattern(path);
    }

    println!("{:?}", network);

    // Test
    let test_pictures_src = vec!["test/A.png", "test/B.png", "test/C.png", "test/D.png"];
    for test_image_src in test_pictures_src {
        let input = image_to_pattern(test_image_src);
        let output = network.feed(&input);

        println!("Input: {:?}", input);
        println!("Output: {:?}", output);
    }
}
