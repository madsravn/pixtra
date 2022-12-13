use crate::pixels::{ColorTrait, Colors, Pixel};
use crate::utility::{clamp, overlap_colors, to_grey_lumiosity};
use image::{GenericImageView, ImageFormat, RgbaImage};
use std::cmp::{max, min};
use std::fmt;
use std::path::Path;

//TODO: Should I use u32 or usize? Rely on image crate?
//
//
//TODO!!!! Make use of Point and Box for get_pixel and get_subimage
//TODO!!!! Create documentation so you can utilize it in this project

#[derive(Clone, Debug)]
pub struct Canvas {
    pixels: Vec<Pixel>,
    height: u32,
    width: u32,
}

#[derive(Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct Rect {
    pub start: Point,
    pub size: Size,
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

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl Eq for Size {}

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
        write!(
            f,
            "Image with {} pixels and dimensions: ({}, {}).",
            self.pixels.len(),
            self.width,
            self.height
        )
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
    /// Create a new `Canvas` of size `width` and `height`
    ///
    /// # Examples
    ///
    /// ```
    /// use pixtra::canvas::Canvas;
    ///
    /// let canvas = Canvas::new(20, 20);
    /// ```
    /// For more examples look at examples (examples/create-image.rs)
    pub fn new(width: u32, height: u32) -> Canvas {
        let width = max(width, 1);
        let height = max(height, 1);
        let pixels = vec![Colors::WHITE; (width * height) as usize];
        Canvas {
            pixels,
            height,
            width,
        }
    }

    /// Creates a new `Canvas` of size `width` and `height` with initial data of `data`
    // TODO: Make it so that we align size of `data` to align with `width*height`.
    pub fn new_with_data(width: u32, height: u32, data: Vec<Pixel>) -> Canvas {
        Canvas {
            width,
            height,
            pixels: data,
        }
    }

    /// Retrieves width and height of canvas in a `Size` struct.
    pub fn dimensions(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Returns an iterator for all the pixels in the `Canvas`.
    // TODO: Should this return the positions of the pixels as well or do we want a utility
    // function for that?
    pub fn pixels(&self) -> std::slice::Iter<'_, Pixel> {
        self.pixels.iter()
    }

    /// Creates a new `Canvas` of size `width` and `height` with all pixels initially set to
    /// `color`.
    pub fn new_with_background(width: u32, height: u32, color: Pixel) -> Canvas {
        let width = max(width, 1);
        let height = max(height, 1);
        let pixels = vec![color; (width * height) as usize];
        Canvas {
            pixels,
            height,
            width,
        }
    }

    /// Saves the canvas as an image at the path given by `filename`
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

    /// Loads the image at the path given by `filename` and returns it as a canvas
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

    /// Counts the amount of pixels in the canvas equal to `pixel`
    pub fn count_pixels(&self, pixel: &Pixel) -> u32 {
        self.find_positions_of_pixels(pixel).len() as u32
    }

    /// Counts the amount of pixels in the canvas that are within the distance of `distance` of
    /// `pixel`
    pub fn count_pixels_with_distance(&self, pixel: &Pixel, distance: f32) -> u32 {
        self.find_positions_of_pixels_with_distance(pixel, distance)
            .len() as u32
    }

    fn find_positions_of_pixels(&self, pixel: &Pixel) -> Vec<usize> {
        self.pixels
            .iter()
            .enumerate()
            .filter(|(_, val)| val == &pixel)
            .map(|(i, _)| i)
            .collect()
    }

    fn find_positions_of_pixels_with_distance(&self, pixel: &Pixel, distance: f32) -> Vec<usize> {
        self.pixels
            .iter()
            .enumerate()
            .filter(|(_, val)| pixel.distance(&val) < distance)
            .map(|(i, _)| i)
            .collect()
    }

    /// Replaces all pixels in the canvas that are within the distance of `distance` of `pixel`
    pub fn replace_pixel_with_distance(
        mut self,
        find_pixel: &Pixel,
        distance: f32,
        replace_pixel: &Pixel,
    ) -> Canvas {
        let positions = self.find_positions_of_pixels_with_distance(find_pixel, distance);
        for pos in positions {
            self.pixels[pos] = replace_pixel.clone();
        }

        self
    }

    /// Replaces all pixels in the canvas that are within the distance of `distance` of `pixel`
    pub fn replace_pixel_with_distance_mut(
        &mut self,
        find_pixel: &Pixel,
        distance: f32,
        replace_pixel: &Pixel,
    ) {
        let positions = self.find_positions_of_pixels_with_distance(find_pixel, distance);
        for pos in positions {
            self.pixels[pos] = replace_pixel.clone();
        }
    }

    /// Replaces all pixels in the canvas that are equal to `pixel`
    pub fn replace_pixel_with(mut self, find_pixel: &Pixel, replace_pixel: &Pixel) -> Canvas {
        let positions = self.find_positions_of_pixels(find_pixel);
        for pos in positions {
            self.pixels[pos] = replace_pixel.clone();
        }

        self
    }

    /// Replaces all pixels in the canvas that are equal to `pixel`
    pub fn replace_pixel_with_mut(&mut self, find_pixel: &Pixel, replace_pixel: &Pixel) {
        let positions = self.find_positions_of_pixels(find_pixel);
        for pos in positions {
            self.pixels[pos] = replace_pixel.clone();
        }
    }

    /// Returns a canvas that is subimage starting at `(x, y)` with size `width x height`.
    pub fn get_subimage(&self, x: u32, y: u32, width: u32, height: u32) -> Canvas {
        let width = min(width, self.width - x);
        let height = min(height, self.height - y);

        let mut c = Canvas::new(width, height);
        for i in 0..width {
            for j in 0..height {
                c.set_pixel_mut(i, j, &self.get_pixel(x + i, y + j));
            }
        }
        c
    }

    /// Inserts canvas `canvas` as a subimage at `(x, y)`
    pub fn set_subimage_mut(&mut self, x: u32, y: u32, canvas: &Canvas) {
        let width = min(canvas.width, self.width - x);
        let height = min(canvas.height, self.height - y);
        for i in 0..width {
            for j in 0..height {
                self.set_pixel_mut(x + i, y + j, &canvas.get_pixel(i, j));
            }
        }
    }

    /// Inserts canvas `canvas` as a subimage at `(x, y)`
    pub fn set_subimage(mut self, x: u32, y: u32, c: &Canvas) -> Canvas {
        let width = min(c.width, self.width - x);
        let height = min(c.height, self.height - y);
        for i in 0..width {
            for j in 0..height {
                self.set_pixel_mut(x + i, y + j, &c.get_pixel(i, j));
            }
        }
        self
    }

    /// Returns pixel at position `(x, y)` from the canvas
    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        let x = clamp(0, self.width - 1, x);
        let y = clamp(0, self.height - 1, y);
        let pixel = self.pixels[(self.width * y + x) as usize].clone();
        pixel
    }

    /// Sets pixel at position `(x, y)` to `pixel`
    pub fn set_pixel(mut self, x: u32, y: u32, pixel: &Pixel) -> Canvas {
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
        self
    }

    /// Sets pixel at position `(x, y)` to `pixel`
    pub fn set_pixel_mut(&mut self, x: u32, y: u32, pixel: &Pixel) {
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
    }

    /// Turns the entire canvas grayscale.
    pub fn to_grey(&self) -> Canvas {
        let pixels = self.pixels.iter().map(|x| to_grey_lumiosity(x)).collect();
        Canvas {
            pixels,
            height: self.height,
            width: self.width,
        }
    }

    /// Turns the entire canvas grayscale.
    pub fn to_grey_mut(&mut self) {
        let pixels = self.pixels.iter().map(|x| to_grey_lumiosity(x)).collect();
        self.pixels = pixels;
    }

    /// Draws a square on the canvas. Draws at position `(x, y)` with size `width x height`. Color
    /// is `color`.
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

    /// Draws a square on the canvas. Draws at position `(x, y)` with size `width x height`. Color
    /// is `color`.
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

    fn within_range(&self, x: i64, y: i64, pos: &(i64, i64)) -> bool {
        let result = pos.0 + x > -1
            && pos.0 + x < self.width.into()
            && pos.1 + y > -1
            && pos.1 + y < self.height.into();

        result
    }

    // TODO: This can be much, much prettier. Rethink
    pub fn fill(mut self, x: u32, y: u32, color: &Pixel) -> Canvas {
        let mut visit_next: Vec<(i64, i64)> = vec![(x.into(), y.into())];
        let mut visited: Vec<(i64, i64)> = Vec::new();
        let mut change_color: Vec<(u32, u32)> = vec![(x, y)];
        let checks: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let find_color = self.get_pixel(x, y);

        // TODO: Can this be done with a iter_mut() or something?
        while !visit_next.is_empty() {
            let pos = visit_next.pop().expect("Should contain an element");
            // TODO: This can be moved into the block
            visited.push(pos);
            for position in checks
                .iter()
                .filter(|c| self.within_range(pos.0, pos.1, c))
                .map(|c| (pos.0 as i64 + c.0, pos.1 as i64 + c.1))
                .map(|p| (self.get_pixel(p.0 as u32, p.1 as u32), p))
                .filter(|(check_pixel, _p)| *check_pixel == find_color)
                .map(|(_, p)| p)
                .filter(|p| !visited.contains(p))
            {
                change_color.push((position.0 as u32, position.1 as u32));
                visit_next.push(position);
            }
        }

        for pos in change_color.iter() {
            self.set_pixel_mut(pos.0, pos.1, color);
        }
        self
    }

    /// Applies filter to entire canvas. `filter` is a function that takes a reference to the
    /// canvas and position `(x, y)` and returns the color which should be set at that position.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixels::Pixel;
    use crate::utility::count_colors;

    #[test]
    fn clean_canvas() {
        let canvas = Canvas::new(20, 20);
        let dimensions = canvas.dimensions();
        assert_eq!(
            dimensions,
            Size {
                width: 20,
                height: 20
            }
        );

        let counts = count_colors(&canvas);
        assert_eq!(counts.keys().len(), 1);
        assert_eq!(counts.get(&Pixel::new(255, 255, 255, 255)), Some(&400));
    }

    #[test]
    fn clean_canvas_with_background() {
        let color = Pixel::random();
        let canvas = Canvas::new_with_background(20, 20, color.clone());
        let dimensions = canvas.dimensions();
        assert_eq!(
            dimensions,
            Size {
                width: 20,
                height: 20
            }
        );

        let counts = count_colors(&canvas);
        assert_eq!(counts.keys().len(), 1);
        assert_eq!(counts.get(&color), Some(&400));
    }
}
