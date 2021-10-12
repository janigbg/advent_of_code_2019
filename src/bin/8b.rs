extern crate advent_of_code_2019;
extern crate image;

use advent_of_code_2019::parser;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    parser::print_args(&args);

    if args.len() < 5 {
        println!("Invalid arguments! (use '-f <filename> <x> <y>'");
        std::process::exit(0);
    }

    let digits: Vec<u32> = parser::parse_digits(&args)
        .iter()
        .map(|d| d.parse().unwrap())
        .collect();

    let (x, y): (usize, usize) = (args[3].parse().unwrap(), args[4].parse().unwrap());

    println!("Dimensions: X = {}, Y = {}", x, y);

    if digits.len() % (x*y) != 0 {
        println!("Image size {} is not a multiple of {} (X * Y)", digits.len(), x*y);
        std::process::exit(0);
    }

    let layers: Vec<&[u32]> = digits
        .chunks(x*y)
        .collect();

    println!("Number of layers: {}", layers.len());

    let mut image: Vec<u32> = Vec::with_capacity(x*y);

    for i in 0..x*y {
        let pixel = layers
            .iter()
            .map(|&layer| layer[i])
            .filter(|&p| p != 2)
            .nth(0)
            .unwrap();

        image.push(pixel);
    }

    for y_pos in 0..y {
        for x_pos in 0..x {
            print!("{}", get_ascii_pixel(image[y_pos*x + x_pos]));
        }
        println!();
    }

    let buffer = image
        .iter()
        .flat_map(get_rgb_pixel)
        .collect::<Vec<u8>>();

    if let Err(err) = image::save_buffer(&Path::new("8b.png"), buffer.as_slice(), x as u32, y as u32, image::RGB(8)) {
        println!("{}", err);
    }
}

fn get_rgb_pixel(v: &u32) -> Vec<u8> {
    match *v {
        0 => vec![0, 0, 0],
        1 => vec![255, 255, 255],
        _ => panic!("Invalid pixel")
    }
}

fn get_ascii_pixel(v: u32) -> char {
    match v {
        0 => ' ',
        1 => '*',//'\u{2588}',
        _ => panic!("Invalid pixel")
    }
}