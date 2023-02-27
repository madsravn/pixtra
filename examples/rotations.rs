use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use std::path::Path;

fn draw_filter(_: &Canvas, x: u32, y: u32) -> Pixel {
    let pixel = Pixel::new(x as u8, y as u8, 0, 255 );
    pixel
}


fn main() {

    let canvas = Canvas::new(100, 100);
    let canvas = canvas.filter(draw_filter);
    canvas.save(Path::new("rotation-before.png")).unwrap();
    let canvas = canvas.rotate90();
    canvas.save(Path::new("rotation-after.png")).unwrap();
}
