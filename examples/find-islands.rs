use pixtra::canvas::Canvas;
use pixtra::pixels::{ColorTrait, Colors};
use pixtra::utility::count_colors;
use std::path::Path;

fn main() {
    let canvas = Canvas::load(Path::new("assets/green_islands.png")).unwrap();
    let islands = canvas.find_islands(&Colors::GREEN);
    println!("Size of islands: {}", islands.len());
    for (i, island) in islands.iter().enumerate() {
        println!("Island {} has {} points", i + 1, island.points.len());
    }
    let count = count_colors(&canvas);
    for (key, value) in count.iter() {
        println!("{}: {}", key, value);
    }

}
