use pixtra::canvas::Canvas;
use std::collections::HashMap;
use pixtra::pixels::{ColorTrait, Colors};
use pixtra::utility::count_colors;
use std::path::Path;

fn main() {
    let canvas = Canvas::load(Path::new("assets/small_green_islands.png")).unwrap();
    let count = count_colors(&canvas);
    for (key, value) in count.iter() {
        println!("{}: {}", key, value);
    }
    let islands = canvas.find_islands(&Colors::GREEN);
    println!("Size of islands: {}", islands.len());
    for (i, island) in islands.iter().enumerate() {
        println!("Island {} has {} points", i + 1, island.points.len());
    }
    let count = count_colors(&canvas);
    for (key, value) in count.iter() {
        println!("{}: {}", key, value);
    }
    let color_one = canvas.get_pixel(islands[0].points[0].x, islands[0].points[0].y);
    let color_two = canvas.get_pixel(islands[1].points[0].x, islands[1].points[0].y);
    println!("Color from island one: {}", color_one);
    println!("Color from island one: {}", color_two);
    let mut hashmap = HashMap::new();
    for c in islands[0].points.iter() {
        hashmap.insert(c, "");
    }
    println!("Island 1 hashmap: {}", hashmap.len());
    let mut hashmap = HashMap::new();
    for c in islands[1].points.iter() {
        hashmap.insert(c, "");
    }
    println!("Island 2 hashmap: {}", hashmap.len());


}
