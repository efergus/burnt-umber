

use derive_more::{Add, Sub};

use three_d::{vec3, Vec3};
use wasm_bindgen::prelude::wasm_bindgen;

use super::cie::XYZ;

fn min(v: Vec3) -> f32 {
    v.x.min(v.y).min(v.z)
}

fn max(v: Vec3) -> f32 {
    v.x.max(v.y).max(v.z)
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Add, Sub, PartialEq)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Add, Sub)]
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

#[wasm_bindgen]
pub fn rgb(r: f32, g: f32, b: f32) -> RGB {
    RGB { r, g, b }
}

#[wasm_bindgen]
pub fn hsv(h: f32, s: f32, v: f32) -> HSV {
    HSV::new(h, s, v)
}

#[wasm_bindgen]
impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        RGB { r, g, b }
    }
    pub fn from_hsv(rgb: RGB) -> RGB {
        RGB::from(rgb)
    }
}

#[wasm_bindgen]
impl HSV {
    pub fn new(h: f32, s: f32, v: f32) -> Self {
        HSV { h, s, v }
    }
    pub fn from_rgb(rgb: RGB) -> HSV {
        HSV::from(rgb)
    }
}

impl From<RGB> for Vec3 {
    fn from(v: RGB) -> Self {
        vec3(v.r, v.g, v.b)
    }
}

impl From<Vec3> for RGB {
    fn from(v: Vec3) -> Self {
        RGB { r: v.x, g: v.y, b: v.z }
    }
}

impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        let v = max(rgb.into());
        let c = v - min(rgb.into());
        let h = if c == 0.0 {
            0.0
        } else if v == rgb.r {
            (rgb.g - rgb.b) / c
        } else if v == rgb.g {
            (rgb.r - rgb.b) / c + 2.0
        } else {
            (rgb.r - rgb.g) / c + 4.0
        }
        .rem_euclid(6.0);
        let s = if v == 0.0 { 0.0 } else { v / c };
        HSV { h, s, v }
    }
}

impl From<HSV> for RGB {
    fn from(value: HSV) -> Self {
        let h = (value.h * 6.0).min(5.9999);
        let c = value.v * value.s;
        let x = c * (1.0 - (h.rem_euclid(2.0) - 1.0).abs());
        let m = value.v - c;
        let p = rgb(m, m, m);
        [
            rgb(c, x, 0.0),
            rgb(x, c, 0.0),
            rgb(0.0, c, x),
            rgb(0.0, x, c),
            rgb(x, 0.0, c),
            rgb(c, 0.0, x),
        ][h.floor() as usize]
            + p
    }
}

impl From<XYZ> for RGB {
    fn from(value: XYZ) -> Self {
        let v = Vec3::from(value);
        let v = XYZ::TO_RGB * v;
        RGB::from(v)
    }
}