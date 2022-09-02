use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;

fn main() {
    let color = Pixel {r: 255, g: 255, b: 0, a: 255};

    // Creates new blank canvas. Blank is white.
    let mut canvas = Canvas::new(10, 10);

    canvas.set_pixel_mut(5, 5, &color);

    let pixel = canvas.get_pixel(5, 5);

    assert_eq!(color, pixel);
    println!("We found color {} and we expected color {}", pixel, color);
}
