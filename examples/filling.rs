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

    let canvas = Canvas::new_with_background(100, 100, color.clone())
        .draw_square(10, 10, 80, 80, &new_color)
        .draw_square(20, 20, 60, 60, &color)
        .fill(1, 1, &Pixel::new(172, 172, 172, 255));
    canvas.save(Path::new("testing.png")).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixtra::utility::count_colors;

    //TODO: With a very low probability, the random colors can be equal.
    #[test]
    fn test_prerequisite() {
        let color_one = Pixel::random();
        let canvas = Canvas::new_with_background(100, 100, color_one.clone());
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_two = Pixel::random();
        let canvas = canvas.draw_square(10, 10, 80, 80, &color_two);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&3600));
        assert_eq!(counts.get(&color_two), Some(&6400));
        assert_eq!(counts.keys().len(), 2);

        let color_three = Pixel::random();
        let canvas = canvas.draw_square(20, 20, 60, 60, &color_three);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&3600));
        assert_eq!(counts.get(&color_two), Some(&2800));
        assert_eq!(counts.get(&color_three), Some(&3600));
        assert_eq!(counts.keys().len(), 3);
    }

    // TODO: Test takes 6 seconds to execute. Fix
    #[test]
    fn test_simplest_filling() {
        let color_one = Pixel::random();
        let canvas = Canvas::new_with_background(100, 100, color_one.clone());
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_two = Pixel::random();
        let canvas = canvas.fill(1, 1, &color_two);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_two), Some(&10000));
        assert_eq!(counts.keys().len(), 1);
    }

    #[test]
    fn test_simple_filling_with_neighboors() {
        let color_one = Pixel::random();
        let canvas = Canvas::new_with_background(100, 100, color_one.clone());
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_two = Pixel::random();
        let canvas_one = canvas.clone().fill(1, 1, &color_two);
        let canvas_two = canvas.clone().fill(2, 2, &color_two);
        assert_eq!(canvas_one, canvas_two);
    }

    #[test]
    fn test_filling_same_images_multiple_times() {
        let color_one = Pixel::random();
        let canvas = Canvas::new_with_background(100, 100, color_one.clone());
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_two = Pixel::random();
        let canvas = canvas.fill(1, 1, &color_two);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_two), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_three = Pixel::random();
        let canvas = canvas.fill(1, 1, &color_three);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_three), Some(&10000));
        assert_eq!(counts.keys().len(), 1);

        let color_four = Pixel::random();
        let canvas = canvas.fill(1, 1, &color_four);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_four), Some(&10000));
        assert_eq!(counts.keys().len(), 1);
    }

    #[test]
    fn test_filling_inner_and_outer() {
        let color_one = Pixel::random();
        let color_two = Pixel::random();
        let color_three = Pixel::random();
        let canvas = Canvas::new_with_background(100, 100, color_one.clone())
            .draw_square(10, 10, 80, 80, &color_two)
            .draw_square(20, 20, 60, 60, &color_three);

        let color_four = Pixel::random();
        let canvas = canvas.fill(1, 1, &color_four);
        let counts = count_colors(&canvas);
        assert_eq!(counts.get(&color_one), None);
        assert_eq!(counts.get(&color_two), Some(&2800));
        assert_eq!(counts.get(&color_three), Some(&3600));
        assert_eq!(counts.get(&color_four), Some(&3600));

        //TODO: Not finished
    }

    // More tests:
    // Doesn't do diagonal.
    // Doesn't fill over same color - if "line" is red and we color the black with a red color.
    // Filling inner and outer of square-in-square
    // Slowly fill an inner-out-square
}
