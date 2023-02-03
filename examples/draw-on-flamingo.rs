use pixtra::canvas::Canvas;
use pixtra::pixels::{ColorTrait, Colors, Pixel};
use pixtra::utility::{diff_debug, error};
use std::path::Path;

fn find_non_zero(p: &Pixel, _x: u32, _y: u32) -> bool {
    p.distance(&Colors::ZERO) > 4.0
}

fn main() {
    println!("Starting");
    let foreground = Canvas::load(Path::new("assets/flamingos.png")).unwrap();
    let background = Canvas::load(Path::new("assets/flamingobg.jpg")).unwrap();

    let result = background.draw_subimage(0, 0, &foreground);

    let compare_image = Canvas::load(Path::new("assets/draw-on-example-result.png")).unwrap();
    let diff = diff_debug(&result, &compare_image);
    diff.save(Path::new("diff_debug_flamingo.out.png")).unwrap();

    result.save(Path::new("flamingo.out.png")).unwrap();

    let non_zero = diff.find_with_predicate(find_non_zero);
    let mut diff_two = Canvas::new(400, 400);
    for e in non_zero.iter() {
        println!("{:?}", e);
        diff_two.set_pixel_mut(e.coordinate.x, e.coordinate.y, &Colors::BLACK);
    }
    diff_two.save(Path::new("diff_two.png")).unwrap();
    println!("Count: {}", non_zero.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blending_on_known_image() {
        let foreground = Canvas::load(Path::new("assets/flamingos.png")).unwrap();
        let background = Canvas::load(Path::new("assets/flamingobg.jpg")).unwrap();

        let result = background.draw_subimage(0, 0, &foreground);

        let compared_image = Canvas::load(Path::new("assets/draw-on-example-result.png")).unwrap();
        let diff = diff_debug(&result, &compared_image);
        let error_count = diff.find_with_predicate(find_non_zero).len();
        assert_eq!(error_count, 0);
    }
}
