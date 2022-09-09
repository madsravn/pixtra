use pixtra::canvas::Canvas;
use pixtra::pixels::{Pixel, PixelBuilder};
use std::path::Path;

fn gaussian_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel (this vector) needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![0.06, 0.13, 0.06, 0.13, 0.24, 0.13, 0.06, 0.13, 0.06];
    let coords = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    // For simplicity, we will leave out the edges of the picture.
    if x > 0 && y > 0 && x < canvas.width - 1 && y < canvas.height - 1 {
        let pixel = apply_filter(canvas, (x, y), &kernel, &coords);
        return pixel;
    }
    canvas.get_pixel(x, y)
}

fn apply_filter(canvas: &Canvas, center: (u32, u32), kernel: &Vec<f32>, coords: &Vec<(i32, i32)>) -> Pixel {
    let scales = kernel
        .iter()
        .zip(coords.iter())
        .fold(Pixel::builder(), |acc, (scale, (x, y))|{
            let pixel = canvas.get_pixel((center.0 as i32 + *x) as u32, (center.1 as i32 + *y) as u32);
                acc + PixelBuilder::from(pixel.r as f32 * scale,
                                         pixel.g as f32 * scale,
                                         pixel.b as f32 * scale,
                                         pixel.a as f32) //TODO:  Is this a good idea?


        });
    scales.build()
}

fn lap_edge_detection_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel (this vector) needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![0.5, 1.0, 0.5, 1.0, -6.0, 1.0, 0.5, 1.0, 0.5];
    let coords = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    // For simplicity, we will leave out the edges of the picture.
    if x > 0 && y > 0 && x < canvas.width - 1 && y < canvas.height - 1 {
        let pixel = apply_filter(canvas, (x, y), &kernel, &coords);
        return pixel;

    }
    canvas.get_pixel(x, y)
}

fn prewitt_edge_detection_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    let kernel_one: Vec<f32> = vec![1f32, 0f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, -1f32];
    let kernel_two: Vec<f32> = vec![1f32, 1f32, 1f32, 0f32, 0f32, 0f32, -1f32, -1f32, -1f32];
    let coords = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];


    if x > 0 && y > 0 && x < canvas.width - 1 && y < canvas.height - 1 {
        let pixel_one = apply_filter(canvas, (x, y), &kernel_one, &coords);
        let pixel_two = apply_filter(canvas, (x, y), &kernel_two, &coords);

        let pixel = PixelBuilder::from(
            ((pixel_one.r as u32 * pixel_one.r as u32 + pixel_two.r as u32 * pixel_two.r as u32) as f64).sqrt() as f32,
            ((pixel_one.g as u32 * pixel_one.g as u32 + pixel_two.g as u32 * pixel_two.g as u32) as f64).sqrt() as f32,
            ((pixel_one.b as u32 * pixel_one.b as u32 + pixel_two.b as u32 * pixel_two.b as u32) as f64).sqrt() as f32,
            255f32
            );
        return pixel.build();

    }
    canvas.get_pixel(x, y)
}

fn inverse(pixel: Pixel) -> Pixel {
    Pixel {
        r: u8::max_value() - pixel.r,
        g: u8::max_value() - pixel.g,
        b: u8::max_value() - pixel.b,
        a: pixel.a,
    }
}

fn inverse_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    inverse(canvas.get_pixel(x, y))
}

fn main() {
    // Gaussian blur
    let canvas = Canvas::load(Path::new("assets/lena.png")).unwrap();
    let gaussian_canvas = canvas.filter(gaussian_filter);
    let _ = gaussian_canvas
        .save(Path::new("gaussian_canvas.png"))
        .unwrap();
    let gaussian_canvas = gaussian_canvas.filter(gaussian_filter);
    let gaussian_canvas = gaussian_canvas.filter(gaussian_filter);
    let gaussian_canvas = gaussian_canvas.filter(gaussian_filter);
    let gaussian_canvas = gaussian_canvas.filter(gaussian_filter);
    let _ = gaussian_canvas
        .save(Path::new("very_gaussian_canvas.png"))
        .unwrap();

    // Chaining filters
    let inverse_gaussian_canvas = canvas.filter(inverse_filter).filter(gaussian_filter);
    let _ = inverse_gaussian_canvas
        .save(Path::new("inverse_gaussian_canvas.png"))
        .unwrap();


    let lap_edge_detection_canvas = canvas.filter(lap_edge_detection_filter);
    let _ = lap_edge_detection_canvas.save(Path::new("lap_edge_detection_filter.png")).unwrap();

    let prewitt_edge_detection_canvas = canvas.filter(prewitt_edge_detection_filter);
    let _ = prewitt_edge_detection_canvas.save(Path::new("prewitt_edge_detection_filter.png")).unwrap();

}
