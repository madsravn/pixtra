use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use pixtra::utility::clamp;
use std::path::Path;

fn gaussian_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    // NOTE: The kernel (this vector) needs to sum to 1.0. Below 1.0 your image will appear darker
    // and above 1.0 it will appear lighter.
    let kernel = vec![0.06, 0.13, 0.06, 0.13, 0.24, 0.13, 0.06, 0.13, 0.06];
    // For simplicity, we will leave out the edges of the picture.
    if x > 0 && y > 0 && x < canvas.width - 1 && y < canvas.height - 1 {
        let out = canvas.get_pixel(x - 1, y - 1).scale(kernel[0])
            + canvas.get_pixel(x, y - 1).scale(kernel[1])
            + canvas.get_pixel(x + 1, y - 1).scale(kernel[2])
            + canvas.get_pixel(x - 1, y).scale(kernel[3])
            + canvas.get_pixel(x, y).scale(kernel[4])
            + canvas.get_pixel(x + 1, y).scale(kernel[5])
            + canvas.get_pixel(x - 1, y + 1).scale(kernel[6])
            + canvas.get_pixel(x, y + 1).scale(kernel[7])
            + canvas.get_pixel(x + 1, y + 1).scale(kernel[8]);
        return out;
    }
    canvas.get_pixel(x, y)
}

fn pixel_from_scale(scales: (f32, f32, f32, f32)) -> Pixel {
    Pixel {
        r: clamp(0u32, u8::max_value() as u32, scales.0 as u32) as u8,
        g: clamp(0u32, u8::max_value() as u32, scales.1 as u32) as u8,
        b: clamp(0u32, u8::max_value() as u32, scales.2 as u32) as u8,
        a: 255,
    }

}

fn apply_filter(canvas: &Canvas, center: (u32, u32), kernel: &Vec<f32>, coords: &Vec<(i32, i32)>) -> Pixel {
    let scales = kernel
        .iter()
        .zip(coords.iter())
        .fold((0f32, 0f32, 0f32, 0f32), |acc, (scale, (x, y))|{
            let pixel = canvas.get_pixel((center.0 as i32 + *x) as u32, (center.1 as i32 + *y) as u32);
            (
                acc.0 + pixel.r as f32 * scale,
                acc.1 + pixel.g as f32 * scale,
                acc.2 + pixel.b as f32 * scale,
                acc.3 + pixel.a as f32 * scale,
                )


        });
    pixel_from_scale(scales)
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

        let pixel = Pixel {
            r: clamp(0 as u32, u8::max_value() as u32, ((pixel_one.r as u32 * pixel_one.r as u32 + pixel_two.r as u32 * pixel_two.r as u32) as f64).sqrt() as u32) as u8,
            g: clamp(0 as u32, u8::max_value() as u32, ((pixel_one.g as u32 * pixel_one.g as u32 + pixel_two.g as u32 * pixel_two.g as u32) as f64).sqrt() as u32) as u8,
            b: clamp(0 as u32, u8::max_value() as u32, ((pixel_one.b as u32 * pixel_one.b as u32 + pixel_two.b as u32 * pixel_two.b as u32) as f64).sqrt() as u32) as u8,
            a: 255,
        };

        return pixel;

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
