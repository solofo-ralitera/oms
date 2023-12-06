use std::io;
use colored::Colorize;
use regex::Regex;
use std::path::PathBuf;
use image::{GenericImageView,Pixel};
use termimage::{ops, util::ANSI_RESET_ATTRIBUTES};


///
/// https://github.com/autozimu/colorizex/blob/master/src/main.rs
/// 
pub fn colorize(line: &str, regex: &Regex, color: (u8, u8, u8)) -> Result<String, io::Error> {
    let mut line = line;
    let mut cline = String::new();
    loop {
        if let Some(mat) = regex.find(line) {
            cline += &line[..mat.start()];
            cline += format!("{}", line[mat.start()..mat.end()].on_truecolor(color.0, color.1, color.2)).as_str();
            line = &line[mat.end()..];
        } else {
            cline += line;
            break;
        }
    }

    Ok(cline)
}


pub fn draw_image(thumb_path: &String, size: (u32, u32)) -> String {
    let image = ("thumb".to_string(), PathBuf::from(thumb_path));
    let format = ops::guess_format(&image).unwrap();
    let img = ops::load_image(&image, format).unwrap();
    let img_s = ops::image_resized_size(img.dimensions(), size, true);
    let resized = ops::resize_image(&img, img_s);

    let (width, height) = resized.dimensions();
    let term_h = height / 2;
    let mut result = String::new();

    for y in 0..term_h {
        let upper_y = y * 2;
        let lower_y = upper_y + 1;

        for x in 0..width {
            let upper_pixel = resized.get_pixel(x, upper_y).to_rgb();
            let lower_pixel = resized.get_pixel(x, lower_y).to_rgb();

            result.push_str(&format!(
                   "\x1B[38;2;{};{};{}m\
                    \x1B[48;2;{};{};{}m\u{2580}",
                   upper_pixel[0],
                   upper_pixel[1],
                   upper_pixel[2],
                   lower_pixel[0],
                   lower_pixel[1],
                   lower_pixel[2]));
        }
        result.push_str(&format!("{}\n", ANSI_RESET_ATTRIBUTES));
    }
    return result;
}