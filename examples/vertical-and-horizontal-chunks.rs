use pixtra::canvas::Canvas;
use pixtra::pixels::Pixel;
use pixtra::utility::count_colors;


fn draw_filter(_: &Canvas, _: u32, y: u32) -> Pixel {
    let y_level = (y / 10 + 1) as u8;
    let pixel = Pixel::new(15 * y_level, 0, 0, 255 );
    pixel
}

// Slice pictures up and stitch them back together.
fn main() {

    let canvas = Canvas::new(100, 100);

    // Let's color it.
    let canvas = canvas.filter(draw_filter);
    let counts = count_colors(&canvas);
    assert_eq!(counts.len(), 10);
    assert_eq!(counts.get(&Pixel::new(30, 0, 0, 255)), Some(&1000));
    assert_eq!(counts.get(&Pixel::new(0, 0, 0, 0)), None);

    let chunks = canvas.vertical_chunks(10);
    for (i, chunk) in chunks.iter().enumerate()  {
        let counts  = count_colors(&chunk);
        assert_eq!(counts.len(), 1, "We are testing chunk {}", i);
        assert_eq!(counts.contains_key(&Pixel::new(((i + 1) * 15) as u8, 0, 0, 255)), true, "We are testing chunk {}", i);
    }
}
