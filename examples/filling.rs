use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use pixtra::utility::count_colors;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let color = Pixel {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    let new_color = Pixel::new(255, 0, 0, 255);

    let canvas = Canvas::new_with_background(100, 100, color.clone())
    .draw_square(10, 10, 80, 80, &new_color)
    .draw_square(20, 20, 60, 60, &color)
    .fill(1, 1, &Pixel::new(172, 172, 172, 255));
    canvas.save(Path::new("testing.png")).unwrap();
}

// TODO: Test by making a color profiler. 
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filling() {
        let canvas = Canvas::new_with_background(100, 100, Pixel::new(0, 0, 255, 255));
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&Pixel::new(0,0,255,255)), Some(&10000));

    }
}


