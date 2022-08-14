use std::path::Path;
use image::{GenericImageView, ImageFormat, RgbaImage, Rgba};
use crate::pixels::{Pixel, Colors, ColorTrait};

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

impl Canvas {
    pub fn new(w: u32, h: u32) -> Canvas {
        // TODO: Do this with the correct constructor
        let ps = vec![Colors::WHITE; (w * h) as usize];
        Canvas { pixels: ps, height: h, width: w }
    }

    pub fn save(&self, filename: &Path) -> Result<(), ImageError> {
        /*let mut img = RgbaImage::new(self.width, self.height);
        for p in self.pixels.iter() {
            img.
        }*/
        let img = RgbaImage::from_vec(self.width, self. height,
                                      self.pixels.iter().flat_map(|x| vec![x.r, x.g, x.b, x.a]).collect());

        match img {
            Some(image) => {
                let res = image.save_with_format(filename, ImageFormat::Png);
                match res {
                    Ok(_) => {
                    return Ok(());
                    },
                    Err(_) => {
                        return Err(ImageError::Encoding(String::from("Shit went wrong")));
                    }
                }
            },
            None => {
                // TODO: What to return here?
                return Ok(());
            }
        }
    }

    pub fn load(filename: &Path) -> Result<Canvas, ImageError> {
        let img = image::open(filename);
        // TODO: Can this be made prettier?
        match img {
            Ok(image) => {
                let (width, height) = image.dimensions();
                let mut vec = Vec::with_capacity((width * height) as usize);
                for(x, y, pixel) in image.pixels() {
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
            },
            Err(e) => {
                match e {
                    // TODO: MAP ERRORS
                    _ => {}
                }
                return Err(ImageError::Decoding(String::from("Something went bad")));
            }
        }
    }
}
