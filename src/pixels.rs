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

    //TODO: Add `add` and `subtract`

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

    pub fn set_red(mut self, red: u8) -> Pixel {
        self.r = red;
        self
    }
    
    pub fn set_green(mut self, green: u8) -> Pixel {
        self.g = green;
        self
    }
    
    pub fn set_blue(mut self, blue: u8) -> Pixel {
        self.b = blue;
        self
    }
    
    pub fn set_alpha(mut self, alpha: u8) -> Pixel {
        self.a = alpha;
        self
    }

    pub fn set_red_mut(&mut self, red: u8) {
        self.r = red;
    }
    
    pub fn set_green_mut(&mut self, green: u8) {
        self.g = green;
    }
    
    pub fn set_blue_mut(&mut self, blue: u8) {
        self.b = blue;
    }
    
    pub fn set_alpha_mut(&mut self, alpha: u8) {
        self.a = alpha;
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

        // Red channel
        let pixel_two = &pixel_one.clone().set_red(156);
        assert_ne!(pixel_one.r, pixel_two.r);
        assert_eq!(pixel_one.g, pixel_two.g);
        assert_eq!(pixel_one.b, pixel_two.b);
        assert_eq!(pixel_one.a, pixel_two.a);
        assert_ne!(&pixel_one, pixel_two);

        // Green channel
        let pixel_two = &pixel_one.clone().set_green(156);
        assert_eq!(pixel_one.r, pixel_two.r);
        assert_ne!(pixel_one.g, pixel_two.g);
        assert_eq!(pixel_one.b, pixel_two.b);
        assert_eq!(pixel_one.a, pixel_two.a);
        assert_ne!(&pixel_one, pixel_two);

        // Blue channel
        let pixel_two = &pixel_one.clone().set_blue(156);
        assert_eq!(pixel_one.r, pixel_two.r);
        assert_eq!(pixel_one.g, pixel_two.g);
        assert_ne!(pixel_one.b, pixel_two.b);
        assert_eq!(pixel_one.a, pixel_two.a);
        assert_ne!(&pixel_one, pixel_two);
        
        // Alpha channel
        let pixel_two = &pixel_one.clone().set_alpha(156);
        assert_eq!(pixel_one.r, pixel_two.r);
        assert_eq!(pixel_one.g, pixel_two.g);
        assert_eq!(pixel_one.b, pixel_two.b);
        assert_ne!(pixel_one.a, pixel_two.a);
        assert_ne!(&pixel_one, pixel_two);
    }

    #[test]
    fn pixel_mut_setters() {
        let mut pixel = Pixel {r: 3, g: 3, b: 3, a: 3};
        assert_eq!(&pixel, &Pixel {r: 3, g: 3, b: 3, a: 3});

        pixel.set_red_mut(4);
        assert_eq!(&pixel, &Pixel {r: 4, g: 3, b: 3, a: 3});

        pixel.set_green_mut(5);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 3, a: 3});

        pixel.set_blue_mut(6);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 6, a: 3});

        pixel.set_alpha_mut(7);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 6, a: 7});
    }

    #[test]
    fn pixel_setters() {
        let pixel = Pixel {r: 3, g: 3, b: 3, a: 3};
        assert_eq!(&pixel, &Pixel {r: 3, g: 3, b: 3, a: 3});

        let pixel = pixel.set_red(4);
        assert_eq!(&pixel, &Pixel {r: 4, g: 3, b: 3, a: 3});

        let pixel = pixel.set_green(5);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 3, a: 3});

        let pixel = pixel.set_blue(6);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 6, a: 3});

        let pixel = pixel.set_alpha(7);
        assert_eq!(&pixel, &Pixel {r: 4, g: 5, b: 6, a: 7});
    }
}
