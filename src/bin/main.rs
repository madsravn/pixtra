use std::path::Path;
use pixtra::canvas::Canvas;
fn main() {
    let canvas = Canvas::new(4,5);
    canvas.save(Path::new("here.png")).expect("To be saved");
    println!("Test");

}
