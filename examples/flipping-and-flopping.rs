use pixtra::canvas::Canvas;
use std::path::Path;

fn main() {
    let canvas = Canvas::load(Path::new("assets/lena.png")).unwrap();

    let flipped = canvas.flip();
    let double_flipped = flipped.flip();

    let _ = flipped.save(Path::new("flipped.png")).unwrap();
    let _ = double_flipped
        .save(Path::new("double_flipped.png"))
        .unwrap();

    let flopped = canvas.flop();
    let double_flopped = flopped.flop();

    let _ = flopped.save(Path::new("flopped.png")).unwrap();
    let _ = double_flopped
        .save(Path::new("double_flopped.png"))
        .unwrap();
}

//TODO: Test by flip and flip - are they equal?
// Test by flipping and matching color profiles and upper line
#[cfg(test)]
mod tests {
    use super::*;
    use pixtra::utility::count_colors;

    //TODO: With a very low probability, the random colors can be equal.
    #[test]
    fn test_prerequisite() {}
}
