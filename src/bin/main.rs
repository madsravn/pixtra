use image::io::Reader;
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

// TODO: Look at this for inspiration: https://imagemagick.org/script/identify.php
fn identify(p: &Path) -> String {
    //TODO: Fix unwrap!
    let reader = Reader::open(p).unwrap().with_guessed_format().unwrap();
    reader
        .format()
        .unwrap()
        .extensions_str()
        .get(0)
        .unwrap()
        .to_owned()
        .to_string()
}
