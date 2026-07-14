use pixtra::canvas::Canvas;
use pixtra::utility::{count_colors, counted_colors_to_html};
use std::path::Path;

// TODO: Turn images grey
// Subtract images
// Error between two images
// Find orphan pixels
// Count colors
// Sub-images
// Flip
// Resize?
// Inverse colors

fn main() {
    let canvas = Canvas::load(&Path::new("testing.png")).unwrap();
    let colors = count_colors(&canvas);
    let output = counted_colors_to_html(&colors);
    println!("{}", output);
    canvas.save(Path::new("here.png")).expect("To be saved");
    println!("Test");
}

