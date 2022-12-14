use crate::canvas::Canvas;
use crate::pixels::Pixel;
use std::cmp;
use std::collections::HashMap;

pub fn to_grey_lightness(p: &Pixel) -> Pixel {
    let lightness = max(p.r, p.g, p.b) + min(p.r, p.g, p.b) / 2;
    from_grey(lightness)
}

pub fn to_grey_average(p: &Pixel) -> Pixel {
    let average = (p.r + p.g + p.b) / 3;
    from_grey(average)
}

pub fn to_grey_lumiosity(p: &Pixel) -> Pixel {
    to_grey_custom(p, 0.21, 0.72, 0.07)
}

// Used for finding error
pub fn to_grey_mean(p: &Pixel) -> Pixel {
    to_grey_custom(p, 0.299, 0.587, 0.114)
}

pub fn to_grey_custom(p: &Pixel, v1: f32, v2: f32, v3: f32) -> Pixel {
    let value = (p.r as f32 * v1 + p.g as f32 * v2 + p.b as f32 * v3) as u8;
    from_grey(value)
}

pub fn from_grey(grey: u8) -> Pixel {
    Pixel {
        r: grey,
        g: grey,
        b: grey,
        a: 255,
    }
}

/// Three way maximum function
pub fn max<T>(v1: T, v2: T, v3: T) -> T
where
    T: Ord,
{
    cmp::max(v1, cmp::max(v2, v3))
}

/// Three way minimum function
pub fn min<T>(v1: T, v2: T, v3: T) -> T
where
    T: Ord,
{
    cmp::min(v1, cmp::min(v2, v3))
}

/// Clamp value `value` to be between `min` and `max`.
/// `min` needs to be less than `max`
pub fn clamp<T: PartialOrd>(min: T, max: T, val: T) -> T {
    assert!(min < max);
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }
    val
}

pub fn diff_squared(p1: &Pixel, p2: &Pixel) -> (u32, u32, u32, u32) {
    (
        (p1.r - p2.r).pow(2).into(),
        (p1.g - p2.g).pow(2).into(),
        (p1.b - p2.b).pow(2).into(),
        (p1.a - p2.a).pow(2).into(),
    )
}

// TODO: We can do this with a fold and hashmap, right?
pub fn count_colors(c: &Canvas) -> HashMap<Pixel, usize> {
    let mut map = HashMap::new();
    for pixel in c.pixels() {
        *map.entry(pixel.clone()).or_insert(0) += 1;
    }
    map
}

pub fn diff(c1: &Canvas, c2: &Canvas) -> Canvas {
    let c = Canvas::new(1, 1);
    c
}

pub fn diff_debug(c1: &Canvas, c2: &Canvas) -> Canvas {
    let c1_dim = c1.dimensions();
    let c2_dim = c2.dimensions();
    let res = if c1_dim.width == c2_dim.width && c1_dim.height == c2_dim.height {
        let pixels = c1
            .pixels()
            .zip(c2.pixels())
            .map(|(p1, p2)| p1.diff(p2))
            .collect();
        Canvas::new_with_data(c1_dim.width, c1_dim.height, pixels)
    } else {
        c1.clone()
    };
    res
}

// https://stackoverflow.com/questions/20271479/what-does-it-mean-to-get-the-mse-mean-error-squared-for-2-images
pub fn error(c1: &Canvas, c2: &Canvas) -> u128 {
    let c1_dim = c1.dimensions();
    let c2_dim = c2.dimensions();
    let width = c1_dim.width.min(c2_dim.width);
    let height = c1_dim.height.min(c2_dim.height);
    let (mut r, mut g, mut b, mut a) = (0u128, 0u128, 0u128, 0u128);
    // TODO: Get iterator from both, zip them and fold and then map
    for x in 0..width {
        for y in 0..height {
            let p1 = c1.get_pixel(x, y);
            let p2 = c2.get_pixel(x, y);
            let diff = p1.diff(&p2);
            r += diff.r.pow(2) as u128;
            g += diff.g.pow(2) as u128;
            b += diff.b.pow(2) as u128;
            a += diff.a.pow(2) as u128;
        }
    }
    (r + g + b + a) / ((4 * width * height) as u128)
}

// TODO: Should these exist here or only in the Canvas as private members?
pub fn apply_alpha_color(color_one: f32, alpha_one: f32, color_two: f32, alpha_two: f32) -> f32 {
    (1.0 - alpha_one) * alpha_two * color_two + alpha_one * color_one
}

pub fn overlap_colors(current_pixel: &Pixel, new_pixel: &Pixel) -> Pixel {
    // TODO !!! FIX !!!
    if new_pixel.a == 255 {
        return new_pixel.clone();
    }
    let pixel_one_normalized = new_pixel.normalize();
    let pixel_two_normalized = current_pixel.normalize();
    let new_a = (1.0 - pixel_one_normalized.3) * pixel_two_normalized.3 + pixel_one_normalized.3;
    let new_r = apply_alpha_color(
        pixel_one_normalized.0,
        pixel_one_normalized.3,
        pixel_two_normalized.0,
        pixel_two_normalized.3,
    ) * new_a;
    let new_g = apply_alpha_color(
        pixel_one_normalized.1,
        pixel_one_normalized.3,
        pixel_two_normalized.1,
        pixel_two_normalized.3,
    ) * new_a;
    let new_b = apply_alpha_color(
        pixel_one_normalized.2,
        pixel_one_normalized.3,
        pixel_two_normalized.2,
        pixel_two_normalized.3,
    ) * new_a;

    // TODO: This might be the culprit - should be "denormalized".
    Pixel::from(new_r, new_g, new_b, new_a)
}
