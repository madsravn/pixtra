use crate::utility::clamp;
use std::fmt;

pub trait ColorTrait {
    const WHITE: Pixel;
    const BLACK: Pixel;
    const RED: Pixel;
    const GREEN: Pixel;
    const BLUE: Pixel;
}

pub struct Colors;

impl ColorTrait for Colors {
    const WHITE: Pixel = Pixel {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    const BLACK: Pixel = Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    const RED: Pixel = Pixel {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    const GREEN: Pixel = Pixel {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    const BLUE: Pixel = Pixel {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
}

//TODO: Should this have a ::new? Can we hide the struct for construction but allow reading? 
#[derive(Hash, Clone, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl Eq for Pixel {}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl Pixel {
    pub fn multiply(&self, x: f32, y: f32, z: f32) -> Pixel {
        Pixel {
            r: clamp(0f32, u8::max_value() as f32, self.r as f32 * x) as u8,
            g: clamp(0f32, u8::max_value() as f32, self.g as f32 * y) as u8,
            b: clamp(0f32, u8::max_value() as f32, self.b as f32 * z) as u8,
            a: self.a,
        }
    }

    pub fn distance(&self, other: &Pixel) -> f32 {
        let dist_r = (self.r as i32 - other.r as i32).pow(2) as f32;
        let dist_g = (self.g as i32 - other.g as i32).pow(2) as f32;
        let dist_b = (self.b as i32 - other.b as i32).pow(2) as f32;
        let dist_a = (self.a as i32 - other.a as i32).pow(2) as f32;
        (dist_r + dist_g + dist_b + dist_a).sqrt()
    }

    pub fn diff(&self, other: &Pixel) -> Pixel {
        Pixel { 
            r: (self.r as i16 - other.r as i16).abs() as u8, 
            g: (self.g as i16 - other.g as i16).abs() as u8, 
            b: (self.b as i16 - other.b as i16).abs() as u8, 
            a: (self.a as i16 - other.a as i16).abs() as u8, 
        }
    }

    pub fn normalize(&self) -> (f32, f32, f32, f32) {
        (
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pixel_it_works() {
        let pixel = Pixel {r: 255, g: 255, b: 255, a: 0};
        assert_eq!(pixel.a, 0);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 255);
        assert_eq!(pixel.b, 255);
    }

    #[test]
    fn pixel_equality_works() {
        let pixel_one = Pixel {r: 155, g: 172, b: 3, a: 255};
        let pixel_two = Pixel {r: 155, g: 172, b: 3, a: 255};
        assert_eq!(pixel_one.r, pixel_two.r);
        assert_eq!(pixel_one.g, pixel_two.g);
        assert_eq!(pixel_one.b, pixel_two.b);
        assert_eq!(pixel_one.a, pixel_two.a);
        assert_eq!(pixel_one, pixel_two);

        // TODO: Test for each component
    }
}
