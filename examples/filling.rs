use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use std::path::Path;

fn main() {
    let color = Pixel {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    let new_color = Pixel::new(255, 0, 0, 255);

    let canvas = Canvas::new_with_background(100, 100, color.clone());
    canvas.save(Path::new("one.png")).unwrap();
    let canvas = canvas.draw_square(10, 10, 80, 80, &new_color);
    canvas.save(Path::new("two.png")).unwrap();
    let canvas = canvas.draw_square(20, 20, 60, 60, &color);
    canvas.save(Path::new("filling.png")).unwrap();
    let canvas = canvas.fill(1, 1, &Pixel::new(172, 172, 172, 255));
    canvas.save(Path::new("testing.png")).unwrap();
}
