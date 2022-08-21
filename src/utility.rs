// http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
// mean = avg[0] * 0.299 + avg[1] * 0.587 + avg[2] * 0.114

//TODO: rename to pixel_utils
//
//
//pub fn lightness, luminosity, average, quad(rename) and custom
//
//
//
//Convert other way is just The quick and dirty approach is to repeat the grayscale intensity for each component of RGB. So, if you have grayscale 120, it translates to RGB (120, 120, 120).

use crate::pixels::Pixel;
use crate::canvas::Canvas;
use std::cmp;
use image::io::Reader;
use std::path::Path;

// TO AND FROM GREY

pub fn to_grey_lightness(p: &Pixel) -> Pixel {
    let lightness = max(p.r, p.g, p.b) + min(p.r, p.g, p.b) / 2;
    from_grey(lightness)
}

pub fn to_grey_average(p: &Pixel) -> Pixel {
    let average = (p.r + p.g + p.b) / 3;
    from_grey(average)
}

pub fn to_grey_lumiosity(p: &Pixel) -> Pixel {
    to_grey_custom_qualifier(p, 0.21, 0.72, 0.07)
}

// Used for finding error
pub fn to_grey_mean(p: &Pixel) -> Pixel {
    to_grey_custom_qualifier(p, 0.299, 0.587, 0.114)
}

// TODO: What the EFFE is it called? 
pub fn to_grey_custom_qualifier(p: &Pixel, v1: f32, v2: f32, v3: f32) -> Pixel {
    let value = (p.r as f32 * v1 + p.g as f32 * v2 + p.b as f32 * v3) as u8;
    from_grey(value)
}

pub fn from_grey(grey: u8) -> Pixel {
    Pixel {
        r: grey,
        g: grey,
        b: grey,
        a: 255
    }
}

pub fn max<T>(v1: T, v2: T, v3: T) -> T where T: Ord {
    cmp::max(v1, cmp::max(v2, v3))
}

pub fn min<T>(v1: T, v2: T, v3: T) -> T where T: Ord {
    cmp::min(v1, cmp::min(v2, v3))
}

// TODO: Make a utility class
pub fn clamp<T: Ord>(min: T, max: T, val: T) -> T {
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }
    val
}

// COMPUTING ERROR

pub fn diff_squared(p1: &Pixel, p2: &Pixel) -> (u32, u32, u32, u32) {
    ((p1.r - p2.r).pow(2).into(), (p1.g - p2.g).pow(2).into(), (p1.b - p2.b).pow(2).into(), (p1.a - p2.a).pow(2).into())
}

pub fn error(c1: &Canvas, c2: &Canvas) -> u64 {
    let width = c1.width.min(c2.width);
    let height = c1.height.min(c2.height);
    for x in 0..width {
        for y in 0..height {
            let p1 = c1.get_pixel(x, y);
            let p2 = c2.get_pixel(x, y);

        }
    }

    5
}




