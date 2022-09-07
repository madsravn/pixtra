use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use std::path::Path;

fn inverse(pixel: Pixel) -> Pixel {
    Pixel {
        r: u8::max_value() - pixel.r,
        g: u8::max_value() - pixel.g,
        b: u8::max_value() - pixel.b,
        a: pixel.a,
    }
}

fn unit_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    canvas.get_pixel(x, y)
}

fn inverse_filter(canvas: &Canvas, x: u32, y: u32) -> Pixel {
    inverse(canvas.get_pixel(x, y))
}

fn main() {
    let canvas = Canvas::load(Path::new("assets/lena.png")).unwrap();
    let filtered_canvas = canvas.filter(unit_filter);
    let inverse_canvas = canvas.filter(inverse_filter);
    let inverse_inverse_canvas = inverse_canvas.filter(inverse_filter);

    assert_eq!(canvas, inverse_inverse_canvas);
    assert_ne!(canvas, inverse_canvas);
    println!(
        "Normal and double inversed canvas are equal: {}",
        canvas == inverse_inverse_canvas
    );
    println!(
        "Normal and single inversed canvas are equal: {}",
        canvas == inverse_canvas
    );

    let _ = filtered_canvas
        .save(Path::new("filtered_canvas.png"))
        .unwrap();
    let _ = inverse_canvas
        .save(Path::new("inverse_canvas.png"))
        .unwrap();
}
