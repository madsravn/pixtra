use pixtra::canvas::Canvas;
use pixtra::pixels::{Pixel, PixelBuilder};
use pixtra::utility::{to_grey_lumiosity, count_colors, counted_colors_to_html};
use std::path::Path;

fn gaussian_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![0.06, 0.13, 0.06, 0.13, 0.24, 0.13, 0.06, 0.13, 0.06];
    let coords = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    // For simplicity, we will leave out the edges of the picture.
    let canvas_size = canvas.dimensions();
    if x > 0 && y > 0 && x < canvas_size.width - 1 && y < canvas_size.height - 1 {
        let pixel = apply_filter(canvas, (x, y), &kernel, &coords);
        return pixel;
    }
    canvas.get_pixel(x, y)
}

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

fn lap_edge_detection_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel (this vector) needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![0.5, 1.0, 0.5, 1.0, -6.0, 1.0, 0.5, 1.0, 0.5];
    let coords = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    // For simplicity, we will leave out the edges of the picture.
    let canvas_size = canvas.dimensions();
    if x > 0 && y > 0 && x < canvas_size.width - 1 && y < canvas_size.height - 1 {
        let pixel = apply_filter(canvas, (x, y), &kernel, &coords);
        return pixel;
    }
    canvas.get_pixel(x, y)
}

fn prewitt_edge_detection_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    let kernel_one: Vec<f32> = vec![1f32, 0f32, -1f32, 1f32, 0f32, -1f32, 1f32, 0f32, -1f32];
    let kernel_two: Vec<f32> = vec![1f32, 1f32, 1f32, 0f32, 0f32, 0f32, -1f32, -1f32, -1f32];
    let coords = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let canvas_size = canvas.dimensions();
    if x > 0 && y > 0 && x < canvas_size.width - 1 && y < canvas_size.height - 1 {
        let pixel_one = apply_filter(canvas, (x, y), &kernel_one, &coords);
        let pixel_two = apply_filter(canvas, (x, y), &kernel_two, &coords);

        let pixel = PixelBuilder::from(
            ((pixel_one.r as u32 * pixel_one.r as u32 + pixel_two.r as u32 * pixel_two.r as u32)
                as f64)
                .sqrt() as f32,
            ((pixel_one.g as u32 * pixel_one.g as u32 + pixel_two.g as u32 * pixel_two.g as u32)
                as f64)
                .sqrt() as f32,
            ((pixel_one.b as u32 * pixel_one.b as u32 + pixel_two.b as u32 * pixel_two.b as u32)
                as f64)
                .sqrt() as f32,
            255f32,
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
    let test_image = Canvas::load(Path::new("assets/IMG_0771.JPG")).unwrap();
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
    let _ = lap_edge_detection_canvas
        .save(Path::new("lap_edge_detection_filter.png"))
        .unwrap();

    let prewitt_edge_detection_canvas = canvas.filter(prewitt_edge_detection_filter);
    let _ = prewitt_edge_detection_canvas
        .save(Path::new("prewitt_edge_detection_filter.png"))
        .unwrap();

    let lap_of_gaussian_filter_canvas = test_image.filter(grey_scale_filter).filter(lap_of_gaussian_filter);
    let counted_colors = count_colors(&lap_of_gaussian_filter_canvas);
    println!("{}", counted_colors_to_html(&counted_colors));
    let _ = lap_of_gaussian_filter_canvas
        .save(Path::new("lap_of_gaussian_edge_detection_filter.png"))
        .unwrap();



}
