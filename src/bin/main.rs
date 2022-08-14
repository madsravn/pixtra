use pixtra::canvas::Canvas;
use std::path::Path;
fn main() {
    let canvas = Canvas::new(4, 5);
    canvas.save(Path::new("here.png")).expect("To be saved");
    println!("Test");
}
