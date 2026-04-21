use egui::Color32;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct HexColor<'a> {
    pub hex: &'a str,
}

impl<'a> From<HexColor<'a>> for Color {
    fn from(value: HexColor) -> Self {
        Color::from(value.hex)
    }
}

impl From<Color32> for Color {
    fn from(value: Color32) -> Self {
        Self {
            r: value.r(),
            g: value.g(),
            b: value.b(),
            a: value.a(),
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        let value = if value.starts_with("0x") {
            &value[2..value.len()]
        } else if value.starts_with("#") {
            &value[1..value.len()]
        } else {
            value
        };
        let r = u8::from_str_radix(&value[0..2], 16).unwrap();
        let g = u8::from_str_radix(&value[2..4], 16).unwrap();
        let b = u8::from_str_radix(&value[4..6], 16).unwrap();
        let a = if value.len() == 8 {
            u8::from_str_radix(&value[6..8], 16).unwrap()
        } else {
            255
        };
        Self { r, g, b, a }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
            a: self.a,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = self.r.saturating_sub(rhs.r);
        self.g = self.g.saturating_sub(rhs.g);
        self.b = self.b.saturating_sub(rhs.b);
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: rhs.r.saturating_add(self.r),
            g: rhs.g.saturating_add(self.g),
            b: rhs.b.saturating_add(self.b),
            a: self.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * rhs) as u8,
            g: (self.g as f32 * rhs) as u8,
            b: (self.b as f32 * rhs) as u8,
            a: self.a,
        }
    }
}
