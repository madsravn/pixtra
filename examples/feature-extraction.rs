use pixtra::canvas::{Canvas, Island};
use pixtra::pixels::{Pixel, PixelBuilder};
use pixtra::utility::{to_grey_lumiosity, count_colors, counted_colors_to_html};
use std::path::Path;

fn lap_of_gaussian_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![
        0, 1, 1, 2, 2, 2, 1, 1, 0,
        1, 2, 4, 5, 5, 5, 4, 2, 1,
        1, 4, 5, 3, 0, 3, 5, 4, 1,
        2, 5, 3, -12, -24, -12, 3, 5, 2,
        2, 5, 0, -24, -40, -24, 0, 5, 2,
        2, 5, 3, -12, -24, -12, 3, 5, 2,
        1, 4, 5, 3, 0, 3, 5, 4, 1,
        1, 2, 4, 5, 5, 5, 4, 2, 1,
        0, 1, 1, 2, 2, 2, 1, 1, 0];
    let kernel = kernel.iter().map(|&x| x as f32).collect();
    let mut coords = vec![];
    for y in -4..5 {
        for x in -4..5 {
            coords.push((x, y));
        }
    }
    // For simplicity, we will leave out the edges of the picture.
    let canvas_size = canvas.dimensions();
    if x > 3 && y > 3 && x < canvas_size.width - 4 && y < canvas_size.height - 4 {
        let pixel = apply_filter(canvas, (x, y), &kernel, &coords);
        return pixel;
    }
    canvas.get_pixel(x, y)
}

fn grey_scale_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    to_grey_lumiosity(&canvas.get_pixel(x, y))
}

fn black_or_white_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    let pixel = canvas.get_pixel(x, y);
    if pixel.r < 128 {
        Pixel::new(0, 0, 0, 255)
    } else {
        Pixel::new(255, 255, 255, 255)
    }
}

fn apply_filter(
    canvas: &Canvas,
    center: (u32, u32),
    kernel: &Vec<f32>,
    coords: &Vec<(i32, i32)>,
) -> Pixel {
    let scales = kernel
        .iter()
        .zip(coords.iter())
        .fold(Pixel::builder(), |acc, (scale, (x, y))| {
            let pixel =
                canvas.get_pixel((center.0 as i32 + *x) as u32, (center.1 as i32 + *y) as u32);
            acc + PixelBuilder::from(
                pixel.r as f32 * scale,
                pixel.g as f32 * scale,
                pixel.b as f32 * scale,
                pixel.a as f32,
            )
        });
    scales.build()
}

// TODO: Parametiser så meget som muligt. Gem alle parametre i filnavnet sådan det kan genskabes
// eller justeres.
fn main() {
    //let images = vec!["IMG_0773", "IMG_0775", "IMG_0776", "IMG_0777"];
    //for image in images.iter() {
        //fix_image(image);
    //}
    let images = vec!["IMG_0776", "IMG_0777"];
    let grapes = "IMG_0775";
    let donut = "IMG_0773";
    for image in images.iter() {
        fix_image_with_map(image);
    }
    fix_image_with_map_grapes(grapes);
    fix_image_with_map_donut(donut);


}
fn fix_image_with_map(image_name: &str) {

    let test_image = Canvas::load(Path::new(&format!("assets/{}.JPG", image_name))).unwrap();
    let canvas = Canvas::load(&Path::new(&format!("{}-draw-islands.png", image_name))).unwrap();


    let empty_pixel = Pixel::new(255, 0, 0, 254);
    let canvas = canvas.fill(100, 100, &empty_pixel);

    let size = test_image.dimensions();
    let mut output_image = Canvas::new_with_background(size.width, size.height, Pixel::new(0, 0, 0, 0));
    for x in 0..size.width {
        for y in 0..size.height {
            let test_pixel = canvas.get_pixel(x, y);
            if test_pixel != empty_pixel {
                output_image.set_pixel_mut(x, y, &test_image.get_pixel(x, y));
            }
        }
    }
    output_image.save(&Path::new(&format!("{}-final-result.png", image_name))).unwrap();

}



fn fix_image_with_map_grapes(image_name: &str) {

    let test_image = Canvas::load(Path::new(&format!("assets/{}.JPG", image_name))).unwrap();
    let canvas = Canvas::load(&Path::new(&format!("{}-draw-islands.png", image_name))).unwrap();


    let empty_pixel = Pixel::new(255, 0, 0, 254);
    let canvas = canvas.fill(100, 100, &empty_pixel); // TODO: Find a smarter way to
                                                                     // indicate that a pixel is
                                                                     // "empty"
    // TODO: This is for the image with the grapes
        let canvas = canvas.fill(836, 3500, &empty_pixel);
        let canvas = canvas.fill(963, 3500, &empty_pixel);
        let canvas = canvas.fill(628, 3811, &empty_pixel);
        let canvas = canvas.fill(800, 3971, &empty_pixel);
        let canvas = canvas.fill(1000, 3900, &empty_pixel);

    let size = test_image.dimensions();
    let mut output_image = Canvas::new_with_background(size.width, size.height, Pixel::new(0, 0, 0, 0));
    for x in 0..size.width {
        for y in 0..size.height {
            let test_pixel = canvas.get_pixel(x, y);
            if test_pixel != empty_pixel {
                output_image.set_pixel_mut(x, y, &test_image.get_pixel(x, y));
            }
        }
    }
    output_image.save(&Path::new(&format!("{}-final-result.png", image_name))).unwrap();

}

fn fix_image_with_map_donut(image_name: &str) {

    let test_image = Canvas::load(Path::new(&format!("assets/{}.JPG", image_name))).unwrap();
    let canvas = Canvas::load(&Path::new(&format!("{}-draw-islands.png", image_name))).unwrap();


    let empty_pixel = Pixel::new(255, 0, 0, 254);
    let canvas = canvas.fill(100, 100, &empty_pixel); // TODO: Find a smarter way to
                                                                     // indicate that a pixel is
                                                                     // "empty"
    let canvas = canvas.fill(1830, 3260, &empty_pixel);

    let size = test_image.dimensions();
    let mut output_image = Canvas::new_with_background(size.width, size.height, Pixel::new(0, 0, 0, 0));
    for x in 0..size.width {
        for y in 0..size.height {
            let test_pixel = canvas.get_pixel(x, y);
            if test_pixel != empty_pixel {
                output_image.set_pixel_mut(x, y, &test_image.get_pixel(x, y));
            }
        }
    }
    output_image.save(&Path::new(&format!("{}-final-result.png", image_name))).unwrap();

}

fn fix_image(image_name: &str) {
    // Gaussian blur
    let test_image = Canvas::load(Path::new(&format!("assets/{}.JPG", image_name))).unwrap();

    let lap_of_gaussian_filter_canvas = test_image.filter(grey_scale_filter).filter(lap_of_gaussian_filter);
    let filtered_canvas = lap_of_gaussian_filter_canvas.filter(black_or_white_filter);

    let islands = filtered_canvas.find_islands(&Pixel::new(255, 255, 255, 255));
    let islands_with_size: Vec<Island> = islands.iter().filter(|x| x.points.len() > 30000).map(|x| x.clone()).collect();

    let mut canvas = Canvas::new_with_background(test_image.dimensions().width, test_image.dimensions().height, Pixel::new(0,0,0,255));
    for island in islands_with_size.iter() {
        canvas.draw_island_mut(&island, &Pixel::new(255, 255, 255, 255));
    }
    canvas.save(&Path::new(&format!("{}-draw-islands.png", image_name))).unwrap();

    let empty_pixel = Pixel::new(255, 0, 0, 254);
    let canvas = canvas.fill(100, 100, &empty_pixel); // TODO: Find a smarter way to
                                                                     // indicate that a pixel is
                                                                     // "empty"
    // TODO: This is for the image with the grapes
    //let canvas = canvas.fill(836, 3500, &empty_pixel);
    //let canvas = canvas.fill(963, 3500, &empty_pixel);
    //let canvas = canvas.fill(628, 3811, &empty_pixel);
    //let canvas = canvas.fill(800, 3971, &empty_pixel);
    //let canvas = canvas.fill(1000, 3900, &empty_pixel);
    //canvas.save(&Path::new("draw-islands-green")).unwrap();

    let size = test_image.dimensions();
    let mut output_image = Canvas::new_with_background(size.width, size.height, Pixel::new(0, 0, 0, 0));
    for x in 0..size.width {
        for y in 0..size.height {
            let test_pixel = canvas.get_pixel(x, y);
            if test_pixel != empty_pixel {
                output_image.set_pixel_mut(x, y, &test_image.get_pixel(x, y));
            }
        }
    }
    output_image.save(&Path::new(&format!("{}-final-result.png", image_name))).unwrap();

    // TODO: Put timers in measuring what part of this is soooo slow.
    // Could be the copying? 


    // TODO: Now we have the outline of the islands. 
    // Copy the islands a blank canvas.
    // Fill from the outside
    // Now you have the entire form of all the islands
    // Transfer the pixels that were not filled from the outside


    //TODO: Ideas to make the algorithm more roburt to "holes":
    // Draw the lines very roughly yourself in a copy of the source image. Use this "rough-image"
    // to generate the edges for the edge detection and feature extraction. Then only use source
    // image when actually copying the pixels.
    //
    // OR
    //
    // Find "Orphan" white pixels in the black lines. This can be done by using a filter. If a
    // 15x15 contains more black than white, it is probably a white pixel that should be black




}
