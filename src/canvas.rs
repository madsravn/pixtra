use crate::pixels::{ColorTrait, Colors, Pixel};
use image::{GenericImageView, ImageFormat, RgbaImage};
use std::path::Path;


//TODO: Should I use u32 or usize? Rely on image crate? 

#[derive(Clone, Debug)]
pub struct Canvas {
    pub pixels: Vec<Pixel>,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug)]
pub enum ImageError {
    Decoding(String),
    Encoding(String),
    Parameter(String),
    Limits(String),
    Unsupported(String),
    IoError(String),
}
fn map_error(error: &image::ImageError) -> ImageError {
    match error {
        image::ImageError::Decoding(e) => {
            return ImageError::Decoding(e.to_string());
        }
        image::ImageError::Encoding(e) => {
            return ImageError::Encoding(e.to_string());
        }
        image::ImageError::Parameter(e) => {
            return ImageError::Parameter(e.to_string());
        }
        image::ImageError::Limits(e) => {
            return ImageError::Limits(e.to_string());
        }
        image::ImageError::Unsupported(e) => {
            return ImageError::Unsupported(e.to_string());
        }
        image::ImageError::IoError(e) => {
            return ImageError::IoError(e.to_string());
        }
    }
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Canvas {
        let ps = vec![Colors::WHITE; (w * h) as usize];
        Canvas {
            pixels: ps,
            height: h,
            width: w,
        }
    }

    pub fn save(&self, filename: &Path) -> Result<(), ImageError> {
        let img = RgbaImage::from_vec(
            self.width,
            self.height,
            self.pixels
                .iter()
                .flat_map(|x| vec![x.r, x.g, x.b, x.a])
                .collect(),
        );

        match img {
            Some(image) => {
                let res = image.save_with_format(filename, ImageFormat::Png);
                match res {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(map_error(&e));
                    }
                }
            }
            None => {
                // TODO: What to return here?
                return Ok(());
            }
        }
    }

    pub fn load(filename: &Path) -> Result<Canvas, ImageError> {
        let img = image::open(filename);
        match img {
            Ok(image) => {
                let (width, height) = image.dimensions();
                let mut vec = Vec::with_capacity((width * height) as usize);
                for (x, y, pixel) in image.pixels() {
                    vec[(width * y + x) as usize] = Pixel {
                        r: pixel[0],
                        g: pixel[1],
                        b: pixel[2],
                        a: pixel[3],
                    }
                }
                return Ok(Canvas {
                    pixels: vec,
                    height,
                    width,
                });
            }
            Err(e) => {
                return Err(map_error(&e));
            }
        }
    }

    // TODO: Figure out a better construct than (x: usize, y: usize). Same for boxes
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let x = clamp(0, self.width - 1, x);
        let y = clamp(0, self.height - 1, y);
        let pixel = self.pixels[(self.width * y + x) as usize].clone();
        pixel
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &Pixel) {
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
    }

    // TODO: What is the opionated solution to this that fits into tiles? 
    // If a user calls this to get something that exceeds width and height?
    /*pub fn get_subimage(&self, x: u32, y: u32, w: u32, h: u32) -> Canvas {


    }*/
}

    // TODO: Make a utility class
    fn clamp<T: Ord>(min: T, max: T, val: T) -> T {
        if val > max {
            return max;
        }
        if val < min {
            return min;
        }
        val
    }
