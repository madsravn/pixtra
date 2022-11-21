use crate::utility::clamp;
use rand::distributions::{Distribution, Uniform};
use std::fmt;
use std::ops::Add;

/// Example colors
pub trait ColorTrait {
    const WHITE: Pixel;
    const BLACK: Pixel;
    const RED: Pixel;
    const GREEN: Pixel;
    const BLUE: Pixel;
}

/// Example colors
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
/// The pixel struct holds the red, green, blue and alpha channel in u8's.
#[derive(Hash, Clone, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// TODO: Can we have a generic either f32 or f64? These but none else
/// The PixelBuilder is a construct which allows for values bigger than u8 and smaller than 0. This
/// is used if you want to add a lot of values together and divide them later or want to use both
/// positive and negative values.
pub struct PixelBuilder {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl PixelBuilder {
    /// Creates a new `PixelBuilder` with initial values `(r, g, b, a) = (0f32, 0f32, 0f32, 0f32)`
    pub fn new() -> PixelBuilder {
        PixelBuilder {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 0f32,
        }
    }

    /// Creates a new `PixelBuilder` with initial values `(r, g, b, a) = (red, green, blue,
    /// alpha)`.
    pub fn from(red: f32, green: f32, blue: f32, alpha: f32) -> PixelBuilder {
        PixelBuilder {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub fn build(&self) -> Pixel {
        Pixel::from(self.r, self.g, self.b, self.a)
    }

    pub fn reset(&self) -> PixelBuilder {
        PixelBuilder::new()
    }
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

impl Add for PixelBuilder {
    type Output = PixelBuilder;

    fn add(self, other: PixelBuilder) -> PixelBuilder {
        PixelBuilder {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl Add<Pixel> for PixelBuilder {
    type Output = PixelBuilder;

    fn add(self, other: Pixel) -> PixelBuilder {
        PixelBuilder {
            r: self.r + other.r as f32,
            g: self.g + other.g as f32,
            b: self.b + other.b as f32,
            a: self.a + other.a as f32,
        }
    }
}

impl Add for Pixel {
    type Output = Pixel;

    fn add(self, other: Pixel) -> Pixel {
        Pixel {
            r: clamp(0u16, u8::max_value() as u16, self.r as u16 + other.r as u16) as u8,
            g: clamp(0u16, u8::max_value() as u16, self.g as u16 + other.g as u16) as u8,
            b: clamp(0u16, u8::max_value() as u16, self.b as u16 + other.b as u16) as u8,
            a: clamp(0u16, u8::max_value() as u16, self.a as u16 + other.a as u16) as u8,
        }
    }
}

impl Add<PixelBuilder> for Pixel {
    type Output = PixelBuilder;

    fn add(self, other: PixelBuilder) -> PixelBuilder {
        PixelBuilder {
            r: self.r as f32 + other.r,
            g: self.g as f32 + other.g,
            b: self.b as f32 + other.b,
            a: self.a as f32 + other.a,
        }
    }
}

impl Pixel {
    pub fn builder() -> PixelBuilder {
        PixelBuilder::new()
    }

    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub fn from(red: f32, green: f32, blue: f32, alpha: f32) -> Pixel {
        Pixel {
            r: clamp(0f32, u8::max_value() as f32, red as f32) as u8,
            g: clamp(0f32, u8::max_value() as f32, green as f32) as u8,
            b: clamp(0f32, u8::max_value() as f32, blue as f32) as u8,
            a: clamp(0f32, u8::max_value() as f32, alpha as f32) as u8,
        }
    }

    pub fn random() -> Pixel {
        let mut rng = rand::thread_rng();
        let random = Uniform::from(0..=255);
        Pixel::new(
            random.sample(&mut rng).into(),
            random.sample(&mut rng).into(),
            random.sample(&mut rng).into(),
            255,
        )
    }

    pub fn multiply(&self, x: f32, y: f32, z: f32) -> Pixel {
        Pixel {
            r: clamp(0f32, u8::max_value() as f32, self.r as f32 * x) as u8,
            g: clamp(0f32, u8::max_value() as f32, self.g as f32 * y) as u8,
            b: clamp(0f32, u8::max_value() as f32, self.b as f32 * z) as u8,
            a: self.a,
        }
    }

    pub fn scale(&self, s: f32) -> Pixel {
        self.multiply(s, s, s)
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

    // TODO: This is ugly. Do we need to implement for &Pixel and &PixelBuilder as well?
    #[test]
    fn pixel_builder_simple() {
        let builder = Pixel::builder();
        let pixel = Pixel {
            r: 1,
            g: 2,
            b: 3,
            a: 4,
        };
        let resulting_pixel = (builder + pixel.clone() + pixel.clone() + pixel.clone()).build();
        assert_eq!(
            resulting_pixel,
            Pixel {
                r: 3,
                g: 6,
                b: 9,
                a: 12
            }
        );
    }

    #[test]
    fn pixel_show_display() {
        let pixel = Pixel {
            r: 140,
            g: 200,
            b: 100,
            a: 240,
        };
        let disp = format!("{}", pixel);
        assert_eq!("(140, 200, 100, 240)", disp);
    }

    #[test]
    fn pixel_it_works() {
        let pixel = Pixel {
            r: 255,
            g: 255,
            b: 255,
            a: 0,
        };
        assert_eq!(pixel.a, 0);
        assert_eq!(pixel.r, 255);
        assert_eq!(pixel.g, 255);
        assert_eq!(pixel.b, 255);
    }

    #[test]
    fn pixel_equality_works() {
        let pixel_one = Pixel {
            r: 155,
            g: 172,
            b: 3,
            a: 255,
        };
        let pixel_two = Pixel {
            r: 155,
            g: 172,
            b: 3,
            a: 255,
        };
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
        let mut pixel = Pixel {
            r: 3,
            g: 3,
            b: 3,
            a: 3,
        };
        assert_eq!(
            &pixel,
            &Pixel {
                r: 3,
                g: 3,
                b: 3,
                a: 3
            }
        );

        pixel.set_red_mut(4);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 3,
                b: 3,
                a: 3
            }
        );

        pixel.set_green_mut(5);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 3,
                a: 3
            }
        );

        pixel.set_blue_mut(6);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 6,
                a: 3
            }
        );

        pixel.set_alpha_mut(7);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 6,
                a: 7
            }
        );
    }

    #[test]
    fn pixel_setters() {
        let pixel = Pixel {
            r: 3,
            g: 3,
            b: 3,
            a: 3,
        };
        assert_eq!(
            &pixel,
            &Pixel {
                r: 3,
                g: 3,
                b: 3,
                a: 3
            }
        );

        let pixel = pixel.set_red(4);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 3,
                b: 3,
                a: 3
            }
        );

        let pixel = pixel.set_green(5);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 3,
                a: 3
            }
        );

        let pixel = pixel.set_blue(6);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 6,
                a: 3
            }
        );

        let pixel = pixel.set_alpha(7);
        assert_eq!(
            &pixel,
            &Pixel {
                r: 4,
                g: 5,
                b: 6,
                a: 7
            }
        );
    }
}
