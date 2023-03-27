use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use pixtra::utility::error;
use std::path::Path;

fn draw_filter(_: &Canvas, x: u32, y: u32) -> Pixel {
    let pixel = Pixel::new(x as u8, y as u8, 0, 255);
    pixel
}

fn main() {
    let canvas = Canvas::new(100, 200);
    let canvas = canvas.filter(draw_filter);
    canvas.save(Path::new("rotation-before-1.png")).unwrap();
    let canvas_old = canvas.clone();
    let canvas = canvas.rotate90();
    canvas.save(Path::new("rotation-after-1.png")).unwrap();
    let canvas = canvas.rotate90();
    let canvas = canvas.rotate90();
    let canvas = canvas.rotate90();
    let rotate_error = error(&canvas_old, &canvas);
    println!("ERROR: {}", rotate_error);

    let canvas = Canvas::new(200, 100);
    let canvas = canvas.filter(draw_filter);
    canvas.save(Path::new("rotation-before-2.png")).unwrap();
    let canvas = canvas.rotate90();
    canvas.save(Path::new("rotation-after-2.png")).unwrap();
}
