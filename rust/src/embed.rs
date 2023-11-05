use std::f32::consts::PI;

use cgmath::{vec2, vec3, InnerSpace};
use palette::{FromColor, LinSrgb, Okhsl, Okhsv, Oklab};
use three_d::Vec3;

use crate::element::coloraxis::Axis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisRepresentation {
    Cylindrical,
    Linear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkRepresentation {
    Scale,
    Clamp,
}

pub trait StaticEmbedding<T = Vec3> {
    fn static_embed(pos: T) -> T;
    fn static_invert(pos: T) -> T;
    fn axis_representation() -> AxisRepresentation {
        AxisRepresentation::Cylindrical
    }
    fn chunk_representation() -> ChunkRepresentation {
        ChunkRepresentation::Scale
    }
}

pub trait Embedding<T = Vec3> {
    fn embed(&self, pos: T) -> T;
    fn invert(&self, pos: T) -> T;
    fn axis_representation(&self) -> AxisRepresentation {
        AxisRepresentation::Cylindrical
    }
    fn chunk_representation(&self) -> ChunkRepresentation {
        ChunkRepresentation::Scale
    }
}

impl<T, U: StaticEmbedding<T>> Embedding<T> for U {
    fn embed(&self, pos: T) -> T {
        Self::static_embed(pos)
    }

    fn invert(&self, pos: T) -> T {
        Self::static_invert(pos)
    }

    fn axis_representation(&self) -> AxisRepresentation {
        Self::axis_representation()
    }

    fn chunk_representation(&self) -> ChunkRepresentation {
        Self::chunk_representation()
    }
}

pub struct IdentityEmbedding {}

impl StaticEmbedding<Vec3> for IdentityEmbedding {
    fn static_embed(pos: Vec3) -> Vec3 {
        pos
    }

    fn static_invert(pos: Vec3) -> Vec3 {
        pos
    }

    fn axis_representation() -> AxisRepresentation {
        AxisRepresentation::Linear
    }
}

pub struct ComposedEmbedding {
    pub a: Box<dyn Embedding<Vec3>>,
    pub b: Box<dyn Embedding<Vec3>>,
    pub axis_representation: AxisRepresentation,
    pub chunk_representation: ChunkRepresentation,
}

impl ComposedEmbedding {
    pub fn new(
        a: Box<dyn Embedding<Vec3>>,
        b: Box<dyn Embedding<Vec3>>,
        axis_representation: AxisRepresentation,
        chunk_representation: ChunkRepresentation,
    ) -> Self {
        Self {
            a,
            b,
            axis_representation,
            chunk_representation,
        }
    }
}

impl Embedding<Vec3> for ComposedEmbedding {
    fn embed(&self, pos: Vec3) -> Vec3 {
        self.a.embed(self.b.embed(pos))
    }

    fn invert(&self, pos: Vec3) -> Vec3 {
        self.b.invert(self.a.invert(pos))
    }

    fn axis_representation(&self) -> AxisRepresentation {
        self.axis_representation
    }

    fn chunk_representation(&self) -> ChunkRepresentation {
        self.chunk_representation
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

impl StaticEmbedding<Vec3> for CylindricalEmbedding {
    fn static_embed(pos: Vec3) -> Vec3 {
        let h = pos.x * PI * 2.0;
        let x = h.cos() * pos.z;
        let z = h.sin() * pos.z;
        let y = pos.y;
        vec3(x, y, z)
    }

    fn static_invert(pos: Vec3) -> Vec3 {
        let r = vec2(pos.x, pos.z).magnitude();
        let h = (-pos.z.atan2(pos.x) / PI / 2.0).rem_euclid(1.0);
        let x = h;
        let y = pos.y;
        let z = r;
        vec3(x, y, z)
    }
}

pub struct OkhsvEmbedding {}

impl StaticEmbedding<Vec3> for OkhsvEmbedding {
    fn static_embed(pos: Vec3) -> Vec3 {
        let hsv = Okhsv::new(pos.x * 360.0, pos.z, pos.y);
        let oklab = Oklab::from_color(hsv);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn static_invert(pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let hsv = Okhsv::from_color(oklab);
        let h = hsv.hue.into_positive_radians() / PI / 2.0;
        let s = hsv.saturation;
        let v = hsv.value;
        vec3(h, v, s)
    }
}

pub struct OkhslEmbedding {}

impl StaticEmbedding<Vec3> for OkhslEmbedding {
    fn static_embed(pos: Vec3) -> Vec3 {
        let hsl: Okhsl = Okhsl::new(pos.x * 360.0, pos.z, pos.y);
        let oklab = Oklab::from_color(hsl);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn static_invert(pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let hsl = Okhsl::from_color(oklab);
        let h = hsl.hue.into_positive_radians() / PI / 2.0;
        let s = hsl.saturation;
        let l = hsl.lightness;
        vec3(h, l, s)
    }
}

pub struct LinSrgbOklabEmbedding {}

impl StaticEmbedding<Vec3> for LinSrgbOklabEmbedding {
    fn static_embed(pos: Vec3) -> Vec3 {
        let lin_srgb = LinSrgb::new(pos.x, pos.y, pos.z);
        let oklab = Oklab::from_color(lin_srgb);
        vec3(oklab.l, oklab.a, oklab.b)
    }

    fn static_invert(pos: Vec3) -> Vec3 {
        let oklab = Oklab::new(pos.x, pos.y, pos.z);
        let lin_srgb = LinSrgb::from_color(oklab);
        let r = lin_srgb.red;
        let g = lin_srgb.green;
        let b = lin_srgb.blue;
        vec3(r, g, b)
    }

    fn chunk_representation() -> ChunkRepresentation {
        ChunkRepresentation::Clamp
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
