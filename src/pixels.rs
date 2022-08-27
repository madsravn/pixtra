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

impl Pixel {
    pub fn multiply(&self, x: f32, y: f32, z: f32) -> Pixel {
        // TODO: Clamp - u8 max
        Pixel {
            r: (self.r as f32 * x) as u8,
            g: (self.g as f32 * y) as u8,
            b: (self.b as f32 * z) as u8,
            a: self.a,
        }
    }

    // TODO: Fix this horrible mess
    pub fn distance(&self, other: &Pixel) -> f32 {
        let dist_r = (self.r as i32 - other.r as i32).pow(2) as f32;
        let dist_g = (self.g as i32 - other.g as i32).pow(2) as f32;
        let dist_b = (self.b as i32 - other.b as i32).pow(2) as f32;
        let dist_a = (self.a as i32 - other.a as i32).pow(2) as f32;
        (dist_r + dist_g + dist_b + dist_a).sqrt()
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
