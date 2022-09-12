use crate::pixels::{ColorTrait, Colors, Pixel};
use crate::utility::{clamp, overlap_colors, to_grey_lumiosity};
use image::{GenericImageView, ImageFormat, RgbaImage};
use std::cmp::{max, min};
use std::fmt;
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

impl PartialEq for Canvas {
    fn eq(&self, other: &Self) -> bool {
        if self.width == other.width && self.height == other.height {
            let result = self
                .pixels
                .iter()
                .zip(other.pixels.iter())
                .fold(true, |acc, (left, right)| acc && left == right);
            return result;
        }
        false
    }
}
impl Eq for Canvas {}

// TODO!
impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "()")
    }
}

// TODO: IoError should be descriptive of which file you are trying to open.
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
        let w = max(w, 1);
        let h = max(h, 1);
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
                let mut vec = vec![Colors::WHITE; (width * height) as usize];
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

    pub fn count_pixels(&self, pixel: &Pixel) -> u32 {
        5
    }

    pub fn replace_pixel_with(mut self, find_pixel: &Pixel, replace_pixel: &Pixel) -> Canvas {

        self
    }

    pub fn replace_pixel_with_mut(&mut self, find_pixel: &Pixel, replace_pixel: &Pixel) {

    }




    // TODO: Pretty names with shadowing
    pub fn get_subimage(&self, x: u32, y: u32, width: u32, height: u32) -> Canvas {
        let w = min(width, self.width - x);
        let h = min(height, self.height - y);

        let mut c = Canvas::new(w, h);
        for i in 0..w {
            for j in 0..h {
                c.set_pixel_mut(i, j, &self.get_pixel(x + i, y + j));
            }
        }
        c
    }

    // TODO: Pretty names with shadowing
    pub fn set_subimage_mut(&mut self, x: u32, y: u32, c: &Canvas) {
        let w = min(c.width, self.width - x);
        let h = min(c.height, self.height - y);
        for i in 0..w {
            for j in 0..h {
                self.set_pixel_mut(x + i, y + j, &c.get_pixel(i, j));
            }
        }
    }

    pub fn set_subimage(mut self, x: u32, y: u32, c: &Canvas) -> Canvas {
        let w = min(c.width, self.width - x);
        let h = min(c.height, self.height - y);
        for i in 0..w {
            for j in 0..h {
                self.set_pixel_mut(x + i, y + j, &c.get_pixel(i, j));
            }
        }
        self
    }

    // TODO: Figure out a better construct than (x: usize, y: usize). Same for boxes
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let x = clamp(0, self.width - 1, x);
        let y = clamp(0, self.height - 1, y);
        let pixel = self.pixels[(self.width * y + x) as usize].clone();
        pixel
    }

    pub fn set_pixel(mut self, x: u32, y: u32, pixel: &Pixel) -> Canvas {
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
        self
    }

    pub fn set_pixel_mut(&mut self, x: u32, y: u32, pixel: &Pixel) {
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
    }

    pub fn to_grey(&self) -> Canvas {
        let pixels = self.pixels.iter().map(|x| to_grey_lumiosity(x)).collect();
        Canvas {
            pixels,
            height: self.height,
            width: self.width,
        }
    }

    pub fn to_grey_mut(&mut self) {
        let pixels = self.pixels.iter().map(|x| to_grey_lumiosity(x)).collect();
        self.pixels = pixels;
    }

    pub fn draw_square_mut(&mut self, x: u32, y: u32, w: u32, h: u32, color: &Pixel) {
        if x < self.width && y < self.height {
            for i in x..min(x + w, self.width) {
                for j in y..min(y + h, self.height) {
                    let current_color = &self.get_pixel(i, j);
                    let new_color = overlap_colors(&current_color, &color);
                    self.set_pixel_mut(i, j, &new_color);
                }
            }
        }
    }

    pub fn draw_square(mut self, x: u32, y: u32, w: u32, h: u32, color: &Pixel) -> Canvas {
        if x < self.width && y < self.height {
            for i in x..min(x + w, self.width) {
                for j in y..min(y + h, self.height) {
                    let current_color = &self.get_pixel(i, j);
                    let new_color = overlap_colors(&current_color, &color);
                    self.set_pixel_mut(i, j, &new_color);
                }
            }
        }
        self
    }

    pub fn filter(&self, filter: fn(&Canvas, u32, u32) -> Pixel) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = filter(&self, x, y);
                canvas.set_pixel_mut(x, y, &pixel);
            }
        }
        canvas
    }

    // TODO: What is the opionated solution to this that fits into tiles?
    // If a user calls this to get something that exceeds width and height?
    /*pub fn get_subimage(&self, x: u32, y: u32, w: u32, h: u32) -> Canvas {


    }*/
}
