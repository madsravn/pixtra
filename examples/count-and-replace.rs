use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use std::path::Path;

fn main() {
    let color = Pixel {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };


    let canvas = Canvas::load(Path::new("assets/mario-yellow.png")).unwrap();
    let find_pixel = canvas.get_pixel(canvas.width / 2, canvas.height / 2);
    let pixel_count = canvas.count_pixels(&find_pixel);
    println!("We found {} pixels with color {}", pixel_count, find_pixel);

    let gray_canvas = canvas.clone().replace_pixel_with(&find_pixel, &color);
    gray_canvas.save(Path::new("mario-gray.png")).unwrap();
    let pixel_count = gray_canvas.count_pixels(&find_pixel);
    println!("We found {} pixels with color {}", pixel_count, find_pixel);


    let distance = 10.0;
    let pixel_count = canvas.count_pixels_with_distance(&find_pixel, distance);
    println!("We found {} pixels within {} distance of color {}", pixel_count, distance, find_pixel);
    let gray_canvas_two = canvas.clone().replace_pixel_with_distance(&find_pixel, distance, &color);


    let pixel_count = gray_canvas_two.count_pixels_with_distance(&find_pixel, distance);
    println!("We found {} pixels within {} distance of color {}", pixel_count, distance, find_pixel);

    gray_canvas_two.save(Path::new("mario-gray-two.png")).unwrap();

    let position = (canvas.width / 2 - 100, canvas.height / 2 - 100);
    let sub_image = canvas.get_subimage(position.0, position.1, 200, 200);
    let sub_image = sub_image.replace_pixel_with(&find_pixel, &color);
    let canvas = canvas.set_subimage(position.0, position.1, &sub_image);
    canvas.save(Path::new("mario-gray-subimage.png")).unwrap();

}
