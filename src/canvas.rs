use crate::pixels::{ColorTrait, Colors, Pixel};
use crate::utility::{clamp, overlap_colors, to_grey_lumiosity};
use image::{GenericImageView, ImageFormat, RgbaImage};
use std::cmp::{max, min};
use std::fmt;
use std::path::Path;

//TODO: Should I use u32 or usize? Rely on image crate?
//
//
//TODO: Chunks and merge functions
//TODO!!!! Make use of Point and Box for get_pixel and get_subimage
//TODO!!!! Create documentation so you can utilize it in this project
//TODO!!!! Try to look at optimization at some point. Try smart tricks and accessing data directly
//instead of `get_pixel`
//
//TODO: Split impl up into more files
//TODO: Make a bin busybox

#[derive(Clone, Debug)]
pub struct Canvas {
    pixels: Vec<Pixel>,
    height: u32,
    width: u32,
}

#[derive(Clone, Copy, Hash, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Debug)]
pub struct PixelWithCoordinate {
    pub coordinate: Point,
    pub pixel: Pixel,
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

#[derive(Clone, Debug)]
pub struct Island {
    pub points: Vec<Point>
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

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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
    /// Creates a new `Canvas` of size `width` and `height`
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
        println!("Saving to: {}", filename.display());
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

    /// Returns the indeces of all pixels that are equal to `pixel`.
    fn find_positions_of_pixels(&self, pixel: &Pixel) -> Vec<usize> {
        self.pixels
            .iter()
            .enumerate()
            .filter(|(_, val)| val == &pixel)
            .map(|(i, _)| i)
            .collect()
    }

    /// Returns the indeces of all pixels that are within a given `distance` to `pixel`.
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

    //TODO: How can we use a subimage_iterator here?
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

    pub fn resize(self, x: u32, y: u32) -> Canvas {
        let mut canvas = Canvas::new(x, y);
        canvas.set_subimage_mut(0, 0, &self);
        canvas
    }

    pub fn resize_mut(&mut self, x: u32, y: u32) {
        let copy = self.clone();
        self.width = x;
        self.height = y;
        self.pixels = vec![Colors::WHITE; (x * y) as usize];
        self.set_subimage_mut(0, 0, &copy);
    }

    pub fn rotate90_mut(&mut self) {
        let prev_dims = self.dimensions();
        if self.width != self.height {
            let max_side = max(self.width, self.height);
            self.resize_mut(max_side, max_side);
        }
        let dimensions = self.dimensions();
        let innerdim = Size {
            width: dimensions.width - 1,
            height: dimensions.height - 1,
        };
        for x in 0..dimensions.width / 2 {
            for y in 0..dimensions.height / 2 {
                let cloned = self.get_pixel(x, y);
                self.set_pixel_mut(x, y, &self.get_pixel(y, innerdim.height - x));
                self.set_pixel_mut(
                    y,
                    innerdim.height - x,
                    &self.get_pixel(innerdim.width - x, innerdim.height - y),
                );
                self.set_pixel_mut(
                    innerdim.width - x,
                    innerdim.height - y,
                    &self.get_pixel(innerdim.width - y, x),
                );
                self.set_pixel_mut(innerdim.width - y, x, &cloned);
            }
        }
        if prev_dims.width > prev_dims.height {
            let canvas = self.get_subimage(
                self.width - prev_dims.height,
                0,
                prev_dims.height,
                prev_dims.width,
            );
            self.resize_mut(prev_dims.height, prev_dims.width);
            self.set_subimage_mut(0, 0, &canvas);
        }
        if prev_dims.height > prev_dims.width {
            let canvas = self.get_subimage(0, 0, prev_dims.height, prev_dims.width);
            self.resize_mut(prev_dims.height, prev_dims.width);
            self.set_subimage_mut(0, 0, &canvas);
        }
    }

    pub fn rotate180_mut(&mut self) {
        self.rotate90_mut();
        self.rotate90_mut();
    }

    pub fn rotate270_mut(&mut self) {
        self.rotate180_mut();
        self.rotate90_mut();
    }

    pub fn rotate90(mut self) -> Canvas {
        let prev_dims = self.dimensions();
        if self.width != self.height {
            let max_side = max(self.width, self.height);
            self.resize_mut(max_side, max_side);
        }
        let dimensions = self.dimensions();
        let innerdim = Size {
            width: dimensions.width - 1,
            height: dimensions.height - 1,
        };
        for x in 0..dimensions.width / 2 {
            for y in 0..dimensions.height / 2 {
                let cloned = self.get_pixel(x, y);
                self.set_pixel_mut(x, y, &self.get_pixel(y, innerdim.height - x));
                self.set_pixel_mut(
                    y,
                    innerdim.height - x,
                    &self.get_pixel(innerdim.width - x, innerdim.height - y),
                );
                self.set_pixel_mut(
                    innerdim.width - x,
                    innerdim.height - y,
                    &self.get_pixel(innerdim.width - y, x),
                );
                self.set_pixel_mut(innerdim.width - y, x, &cloned);
            }
        }
        if prev_dims.width > prev_dims.height {
            self = self.get_subimage(
                self.width - prev_dims.height,
                0,
                prev_dims.height,
                prev_dims.width,
            );
        }
        if prev_dims.height > prev_dims.width {
            self = self.get_subimage(0, 0, prev_dims.height, prev_dims.width);
        }
        self
    }

    pub fn rotate180(self) -> Canvas {
        self.rotate90().rotate90()
    }

    pub fn rotate270(self) -> Canvas {
        self.rotate180().rotate90()
    }

    pub fn vertical_chunks(&self, size_of_chunk: u32) -> Vec<Canvas> {
        let chunks: Vec<Canvas> = self
            .pixels
            .chunks((size_of_chunk * self.width) as usize)
            .map(|x| Canvas::new_with_data(self.width, size_of_chunk, x.to_vec()))
            .collect();

        chunks
    }

    //Ugly version
    //Clone?
    pub fn horizontal_chunks(&self, size_of_chunk: u32) -> Vec<Canvas> {
        let chunks: Vec<Canvas> = self
            .clone()
            .rotate90()
            .pixels
            .chunks((size_of_chunk * self.width) as usize)
            .map(|x| Canvas::new_with_data(self.width, size_of_chunk, x.to_vec()))
            .map(|x| x.rotate270())
            .collect();

        chunks
    }

    fn index_to_coordinate(&self, index: u32) -> Point {
        let (x, y) = (index % self.width, index / self.width);
        Point {x, y}
    }

    pub fn iter(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.pixels.iter().map(|x| x.clone())
    }

    pub fn iter_with_coordinates(&self) -> impl Iterator<Item = PixelWithCoordinate> + '_ {
        let range = 0..self.pixels.len();
        let range = range.map(|x| self.index_to_coordinate(x as u32));
        self.pixels
            .iter()
            .zip(range)
            .map(|(pixel, point)| PixelWithCoordinate {
                coordinate: point,
                pixel: pixel.clone(),
            })
    }

    fn subimage_iterator<'a, 'b>(
        &'a self,
        x: u32,
        y: u32,
        canvas: &'b Canvas,
    ) -> impl Iterator<Item = PixelWithCoordinate> + '_
    where
        'b: 'a,
    {
        let width = min(canvas.width, self.width - x);
        let height = min(canvas.height, self.height - y);

        canvas
            .iter_with_coordinates()
            .filter(move |p| p.coordinate.x < width && p.coordinate.y < height)
            .map(move |p| PixelWithCoordinate {
                coordinate: Point {
                    x: p.coordinate.x + x,
                    y: p.coordinate.y + y,
                },
                pixel: p.pixel.clone(),
            })
    }

    /// Draws canvas `canvas` as a subimage at `(x, y)`
    pub fn draw_subimage_mut(&mut self, x: u32, y: u32, canvas: &Canvas) {
        // This is ugly. Fix it
        let binding = self.clone();
        let iter = binding.subimage_iterator(x, y, canvas);
        for pixelwithcoordinate in iter {
            let destination = self.get_pixel(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
            );
            let source = pixelwithcoordinate.pixel;
            let new_color = overlap_colors(&destination, &source);
            self.set_pixel_mut(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
                &new_color,
            );
        }
    }

    /// Inserts canvas `canvas` as a subimage at `(x, y)`
    pub fn set_subimage_mut(&mut self, x: u32, y: u32, canvas: &Canvas) {
        let binding = self.clone();
        let iter = binding.subimage_iterator(x, y, canvas);
        for pixelwithcoordinate in iter {
            self.set_pixel_mut(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
                &pixelwithcoordinate.pixel,
            );
        }
    }

    // TODO: Four parameters? Ugly
    pub fn trace(mut self, island: &Island, color: &Pixel, left: u32, right: u32, up: u32, down: u32) -> Canvas {
        let left: i64 = left.into();
        let right: i64 = right.into();
        let up: i64 = up.into();
        let down: i64 = down.into();

        for point in island.points.iter() {

            for x in -left..=right {
                for y in -up..=down {
                    self.set_pixel_mut_signed(point.x as i64 + x, point.y as i64 + y, &color);
                }
            }
        }

        self
    }

    pub fn trace_mut(&mut self, island: &Island, color: &Pixel, left: u32, right: u32, up: u32, down: u32) {
        let left: i64 = left.into();
        let right: i64 = right.into();
        let up: i64 = up.into();
        let down: i64 = down.into();

        for point in island.points.iter() {

            for x in -left..=right {
                for y in -up..=down {
                    self.set_pixel_mut_signed(point.x as i64 + x, point.y as i64 + y, &color);
                }
            }
        }

    }

    pub fn draw_island(mut self, island: &Island, color: &Pixel) -> Canvas {
        for point in island.points.iter() {
            self.set_pixel_mut(point.x, point.y, color);
        }

        self
    }

    pub fn draw_island_mut(&mut self, island: &Island, color: &Pixel) {
        for point in island.points.iter() {
            self.set_pixel_mut(point.x, point.y, color);
        }

    }

    /// Draws canvas `canvas` as a subimage at `(x, y)`
    pub fn draw_subimage(mut self, x: u32, y: u32, canvas: &Canvas) -> Canvas {
        let binding = self.clone();
        let iter = binding.subimage_iterator(x, y, canvas);
        for pixelwithcoordinate in iter {
            let destination = self.get_pixel(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
            );
            let source = pixelwithcoordinate.pixel;
            let new_color = overlap_colors(&destination, &source);
            self.set_pixel_mut(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
                &new_color,
            );
        }
        self
    }

    /// Inserts canvas `canvas` as a subimage at `(x, y)`
    pub fn set_subimage(mut self, x: u32, y: u32, canvas: &Canvas) -> Canvas {
        let binding = self.clone();
        let iter = binding.subimage_iterator(x, y, canvas);
        for pixelwithcoordinate in iter {
            self.set_pixel_mut(
                pixelwithcoordinate.coordinate.x,
                pixelwithcoordinate.coordinate.y,
                &pixelwithcoordinate.pixel,
            );
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
        // TODO: We need to make sure that (x, y) is within the border
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
        self
    }

    /// Mutable sets pixel at position `(x, y)` to `pixel`
    pub fn set_pixel_mut(&mut self, x: u32, y: u32, pixel: &Pixel) {
        // TODO: We need to make sure that (x, y) is within the border
        self.pixels[(self.width * y + x) as usize] = pixel.clone();
    }

    /// Mutable sets pixel at position `(x, y)` to `pixel` if `x: i64` and `y: i64` is within image
    /// bounds
    pub fn set_pixel_mut_signed(&mut self, x: i64, y: i64, pixel: &Pixel) {
        if self.in_bounds(x, y) {
            self.set_pixel_mut(x as u32, y as u32, pixel);
        }
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
        let canvas = Canvas::new_with_background(w, h, color.clone());
        self.draw_subimage_mut(x, y, &canvas);
    }

    /// Draws a square on the canvas. Draws at position `(x, y)` with size `width x height`. Color
    /// is `color`.
    pub fn draw_square(mut self, x: u32, y: u32, w: u32, h: u32, color: &Pixel) -> Canvas {
        let canvas = Canvas::new_with_background(w, h, color.clone());
        self.draw_subimage_mut(x, y, &canvas);

        self
    }

    // TODO: When using for feature extraction, remember to filter to black and white only
    pub fn find_islands(&self, island_color: &Pixel) -> Vec<Island> {
        let mut canvas = self.clone();
        let points = canvas.find_colors(island_color);
        let islands: Vec<Island> = points.iter().map(|x| canvas.find_islands_with_fill(x.x, x.y, &Colors::BLACK)).flatten().collect();

        islands
    }

    fn find_colors(&self, color: &Pixel) -> Vec<Point> {
        let points: Vec<Point> = self.pixels.iter().enumerate().filter(|(_, p)| p == &color).map(|(i, _)| self.index_to_coordinate(i as u32)).collect();
        points
    }

    // By orlp
    fn in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width.into() && y >= 0 && y < self.height.into()
    }

    // By orlp
    fn try_get_pixel(&self, x: i64, y: i64) -> Option<&Pixel> {
        self.in_bounds(x, y)
            .then(|| &self.pixels[(self.width as i64 * y + x) as usize])
    }

    pub fn fill_by_distance(mut self, x: u32, y: u32, fill_color: &Pixel, distance: f32) -> Canvas {
        let find_color = self.get_pixel(x, y);
        self = self.fill_by_color_and_distance(x, y, fill_color, &find_color, distance);
        self
    }

    pub fn fill_by_color_and_distance(mut self, x: u32, y: u32, fill_color: &Pixel, color: &Pixel, distance: f32) -> Canvas {
        let find_color = color.clone();
        // TODO: This might not work if you hit this exact color by accident?
        if fill_color == &find_color {
            return self;
        }

        let mut to_visit = vec![(x as i64, y as i64)];
        while let Some((x, y)) = to_visit.pop() {
            self.set_pixel_mut(x as u32, y as u32, fill_color);

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                match self.try_get_pixel(x + dx, y + dy) {
                    Some(&ref p) => {
                        if p.distance(&find_color) < distance {
                            to_visit.push((x + dx, y + dy));
                        }
                    },
                    _ => {}
                }
            }
        }
        self
    }

    fn find_islands_with_fill(&mut self, x: u32, y: u32, fill_color: &Pixel) -> Option<Island> {
        let mut points = vec![];
        let find_color = self.get_pixel(x, y);
        if fill_color == &find_color {
            return None;
        }

        let mut to_visit = vec![(x as i64, y as i64)];
        while let Some((x, y)) = to_visit.pop() {
            let point = Point {x: x as u32, y: y as u32};
            if !points.contains(&point) {
                self.set_pixel_mut(x as u32, y as u32, fill_color);
                points.push(Point { x: x as u32, y: y as u32});
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if self.try_get_pixel(x + dx, y + dy) == Some(&find_color) {
                        to_visit.push((x + dx, y + dy));
                    }
                }
            }
        }
        let island = Some(Island {points}); 
        island
    }



    // By orlp
    pub fn fill(mut self, x: u32, y: u32, fill_color: &Pixel) -> Canvas {
        let find_color = self.get_pixel(x, y);
        if fill_color == &find_color {
            return self;
        }

        let mut to_visit = vec![(x as i64, y as i64)];
        while let Some((x, y)) = to_visit.pop() {
            self.set_pixel_mut(x as u32, y as u32, fill_color);

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if self.try_get_pixel(x + dx, y + dy) == Some(&find_color) {
                    to_visit.push((x + dx, y + dy));
                }
            }
        }
        self

    }

    /// Applies filter to entire canvas. `filter` is a function that takes a reference to the
    /// canvas and position `(x, y)` and returns the color which should be set at that position.
    pub fn filter(&self, filter: fn(&Canvas, u32, u32) -> Pixel) -> Canvas {
        // TODO: Do this with an iterator instead
        let mut canvas = Canvas::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let pixel = filter(&self, x, y);
                canvas.set_pixel_mut(x, y, &pixel);
            }
        }
        canvas
    }

    /// Finds all pixels where predicate `predicate` holds
    pub fn find_with_predicate(
        &self,
        predicate: fn(&Pixel, u32, u32) -> bool,
    ) -> Vec<PixelWithCoordinate> {
        let result = self
            .iter_with_coordinates()
            .filter(|x| predicate(&x.pixel, x.coordinate.x, x.coordinate.y))
            .collect();

        result

        /*
        let mut vec = Vec::new();
        //TODO: Create iterator
        for i in 0..self.width {
            for j in 0..self.height {
                if predicate(&self.get_pixel(i, j), i, j) {
                    let p = PixelWithCoordinate {
                        coordinate: Point { x: i, y: j },
                        pixel: self.get_pixel(i, j).clone(),
                    };
                    vec.push(p);
                }
            }
        }
        vec
        */
    }

    // TODO: What is the opionated solution to this that fits into tiles?
    // If a user calls this to get something that exceeds width and height?
    /*pub fn get_subimage(&self, x: u32, y: u32, w: u32, h: u32) -> Canvas {


    }*/

    /// Flips the image on the vertical axis
    pub fn flip(&self) -> Canvas {
        let mut reversed = Vec::with_capacity(self.width as usize * self.height as usize);
        for pixels in self.pixels.chunks(self.width as usize) {
            let rev: Vec<Pixel> = pixels.iter().rev().map(|x| x.to_owned()).collect();
            reversed.extend(rev);
        }
        Canvas {
            pixels: reversed,
            width: self.width,
            height: self.height,
        }
    }

    /// Flips the image on the horizontal axis
    pub fn flop(&self) -> Canvas {
        let reversed = self.pixels.iter().rev().map(|x| x.to_owned()).collect();

        let canvas = Canvas {
            pixels: reversed,
            width: self.width,
            height: self.height,
        };
        let flipped = canvas.flip();
        flipped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixels::Pixel;
    use crate::utility::count_colors;

    // Stupid function for comparing new iterator
    fn draw_subimage_mut_old(draw_on: &mut Canvas, x: u32, y: u32, canvas: &Canvas) {
        let width = min(canvas.width, draw_on.width - x);
        let height = min(canvas.height, draw_on.height - y);
        for i in 0..width {
            for j in 0..height {
                let destination = draw_on.get_pixel(x + i, y + j);
                let source = canvas.get_pixel(i, j);
                let new_color = overlap_colors(&destination, &source);
                draw_on.set_pixel_mut(x + i, y + j, &new_color);
            }
        }
    }

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
    fn testing_new_iterators() {
        let canvas = Canvas::new(20, 20);
        let filled_canvas = canvas.fill(0, 0, &Pixel::new(255, 255, 0, 255));
        let mut canvas_one = Canvas::new(40, 40);
        let mut canvas_two = Canvas::new(40, 40);
        canvas_one.draw_subimage_mut(10, 10, &filled_canvas);
        draw_subimage_mut_old(&mut canvas_two, 10, 10, &filled_canvas);

        assert_eq!(canvas_one, canvas_two);
    }

    #[test]
    fn testing_new_iterators_out_of_border_y() {
        let position = Point { x: 10, y: 30 };
        let size = 40;
        let canvas = Canvas::new(20, 20);
        let filled_canvas = canvas.fill(0, 0, &Pixel::new(255, 255, 0, 255));
        let mut canvas_one = Canvas::new(size, size);
        let mut canvas_two = Canvas::new(size, size);
        canvas_one.draw_subimage_mut(position.x, position.y, &filled_canvas);
        draw_subimage_mut_old(&mut canvas_two, position.x, position.y, &filled_canvas);

        assert_eq!(canvas_one, canvas_two);
    }

    #[test]
    fn testing_new_iterators_out_of_border_x() {
        let position = Point { x: 30, y: 10 };
        let size = 40;
        let canvas = Canvas::new(20, 20);
        let filled_canvas = canvas.fill(0, 0, &Pixel::new(255, 255, 0, 255));
        let mut canvas_one = Canvas::new(size, size);
        let mut canvas_two = Canvas::new(size, size);
        canvas_one.draw_subimage_mut(position.x, position.y, &filled_canvas);
        draw_subimage_mut_old(&mut canvas_two, position.x, position.y, &filled_canvas);

        assert_eq!(canvas_one, canvas_two);
    }

    #[test]
    fn test_find_with_predicate() {
        let position = Point { x: 10, y: 10 };
        let size = 40;
        let canvas = Canvas::new(20, 20);
        let filled_canvas = canvas.fill(0, 0, &Pixel::new(255, 255, 0, 255));
        let mut canvas_one = Canvas::new(size, size);
        canvas_one.draw_subimage_mut(position.x, position.y, &filled_canvas);
        let predicate = |pixel: &Pixel, _: u32, _: u32| -> bool {
            if pixel == &Pixel::new(255, 255, 0, 255) {
                return true;
            } else {
                return false;
            }
        };
        let found = canvas_one.find_with_predicate(predicate);
        // We filled the canvas with 20 * 20 yellow pixels
        assert_eq!(found.len(), 20 * 20);
        let correct_color = found
            .iter()
            .all(|pixelwithcoordinate| pixelwithcoordinate.pixel == Pixel::new(255, 255, 0, 255));
        assert_eq!(correct_color, true);
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
