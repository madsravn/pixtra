use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;

fn main() {
    let color = Pixel {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };

    // Creates new blank canvas. Blank is white.
    let mut canvas = Canvas::new(10, 10);

    canvas.set_pixel_mut(5, 5, &color);

    let pixel = canvas.get_pixel(5, 5);

    assert_eq!(color, pixel);
    println!("We found color {} and we expected color {}", pixel, color);

    // We can do the same without the _mut modifier
    let color = Pixel {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };

    // Creates new blank canvas. Blank is white.
    let canvas = Canvas::new(10, 10);

    // Here is the difference.
    // We discard the old canvas and use the new one which contains the new pixel
    let canvas = canvas.set_pixel(5, 5, &color);

    let pixel = canvas.get_pixel(5, 5);

    assert_eq!(color, pixel);
    println!("We found color {} and we expected color {}", pixel, color);
}
