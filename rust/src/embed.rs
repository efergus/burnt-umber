use std::f32::consts::PI;

use cgmath::{vec2, vec3, InnerSpace};
use palette::{FromColor, LinSrgb, Okhsv, Oklab, Okhsl};
use three_d::Vec3;

use crate::element::coloraxis::Axis;

pub trait Embedding<T = Vec3> {
    fn embed(&self, pos: T) -> T;
    fn invert(&self, pos: T) -> T;
}

pub struct IdentityEmbedding {}

impl Embedding<Vec3> for IdentityEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        pos
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        pos
    }
}

pub struct ComposedEmbedding {
    pub a: Box<dyn Embedding<Vec3>>,
    pub b: Box<dyn Embedding<Vec3>>,
}

impl ComposedEmbedding {
    pub fn new(a: Box<dyn Embedding<Vec3>>, b: Box<dyn Embedding<Vec3>>) -> Self {
        Self { a, b }
    }
}

impl Embedding<Vec3> for ComposedEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        self.a.embed(self.b.embed(pos))
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        self.b.invert(self.a.invert(pos))
    }
}

pub struct SwapAxesEmbedding {
    pub a1: Axis,
    pub a2: Axis,
}

impl SwapAxesEmbedding {
    pub fn new(a1: Axis, a2: Axis) -> Self {
        Self { a1, a2 }
    }
}

impl Embedding<Vec3> for SwapAxesEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        let mut pos = pos;
        let p: &mut [f32; 3] = pos.as_mut();
        p.swap(self.a1 as usize, self.a2 as usize);
        pos
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        let mut pos = pos;
        let p: &mut [f32; 3] = pos.as_mut();
        p.swap(self.a1 as usize, self.a2 as usize);
        pos
    }
}

pub struct CylindricalEmbedding {}

impl Embedding<Vec3> for CylindricalEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        let h = pos.x * PI * 2.0;
        let x = h.cos() * pos.z;
        let z = h.sin() * pos.z;
        let y = pos.y;
        vec3(x, y, z)
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        let r = vec2(pos.x, pos.z).magnitude();
        let h = (-pos.z.atan2(pos.x) / PI / 2.0).rem_euclid(1.0);
        let x = h;
        let y = pos.y;
        let z = r;
        vec3(x, y, z)
    }
}

pub struct OkhsvEmbedding {}

impl Embedding<Vec3> for OkhsvEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        let hsv = Okhsv::new(pos.x * 360.0, pos.z, pos.y);
        let oklab = Oklab::from_color(hsv);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let hsv = Okhsv::from_color(oklab);
        let h = hsv.hue.into_positive_radians() / PI / 2.0;
        let s = hsv.saturation;
        let v = hsv.value;
        vec3(h, v, s)
    }
}

pub struct OkhslEmbedding {}

impl Embedding<Vec3> for OkhslEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        let hsl: Okhsl = Okhsl::new(pos.x * 360.0, pos.z, pos.y);
        let oklab = Oklab::from_color(hsl);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let hsl = Okhsl::from_color(oklab);
        let h = hsl.hue.into_positive_radians() / PI / 2.0;
        let s = hsl.saturation;
        let l = hsl.lightness;
        vec3(h, l, s)
    }
}

pub struct LinSrgbOklabEmbedding {}

impl Embedding<Vec3> for LinSrgbOklabEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        let lin_srgb = LinSrgb::new(pos.x, pos.y, pos.z);
        let oklab = Oklab::from_color(lin_srgb);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let lin_srgb = LinSrgb::from_color(oklab);
        let r = lin_srgb.red;
        let g = lin_srgb.green;
        let b = lin_srgb.blue;
        vec3(r, g, b)
    }
}

pub struct FnEmbedding<F1, F2>
where
    F1: Fn(Vec3) -> Vec3,
    F2: Fn(Vec3) -> Vec3,
{
    embed: F1,
    invert: F2,
}

impl<F1, F2> FnEmbedding<F1, F2>
where
    F1: Fn(Vec3) -> Vec3,
    F2: Fn(Vec3) -> Vec3,
{
    fn _new(embed: F1, invert: F2) -> Self {
        Self { embed, invert }
    }
}

impl<F1, F2> Embedding<Vec3> for FnEmbedding<F1, F2>
where
    F1: Fn(Vec3) -> Vec3,
    F2: Fn(Vec3) -> Vec3,
{
    fn embed(&self, pos: Vec3) -> Vec3 {
        (self.embed)(pos)
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        (self.invert)(pos)
    }
}
