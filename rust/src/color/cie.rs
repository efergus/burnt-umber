use derive_more::{Add, Sub};
use three_d::{Mat3, SquareMatrix, Vec3, vec3};

#[derive(Debug, Clone, Copy, PartialEq, Add, Sub)]
pub struct XYY {
    x: f32,
    y: f32,
    #[allow(non_snake_case)]
    Y: f32,
}

fn xyz(x: f32, y: f32, z: f32) -> XYZ {
    XYZ { x, y, z }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Sub)]
pub struct XYZ {
    x: f32,
    y: f32,
    z: f32,
}

impl XYZ {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        XYZ { x, y, z }
    }
}

impl From<XYZ> for Vec3 {
    fn from(value: XYZ) -> Self {
        vec3(value.x, value.y, value.z)
    }
}

impl From<Vec3> for XYZ {
    fn from(value: Vec3) -> Self {
        XYZ {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

pub struct Lab {
    #[allow(non_snake_case)]
    L: f32,
    a: f32,
    b: f32,
}

pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

impl From<RGB> for Vec3 {
    fn from(value: RGB) -> Self {
        vec3(value.r, value.g, value.b)
    }
}

impl From<Vec3> for RGB {
    fn from(value: Vec3) -> Self {
        RGB {
            r: value.x,
            g: value.y,
            b: value.z,
        }
    }
}

impl From<XYZ> for XYY {
    fn from(value: XYZ) -> Self {
        let sum = value.x + value.y + value.z;
        if sum == 0.0 {
            XYY {
                x: 0.0,
                y: 0.0,
                Y: 0.0,
            }
        } else {
            XYY {
                x: value.x / sum,
                y: value.y / sum,
                Y: value.y,
            }
        }
    }
}

// TODO: Check this
const D65: XYZ = XYZ {
    x: 0.95047,
    y: 1.00000,
    z: 1.08883,
};

// TODO: Check this
const D50: XYZ = XYZ {
    x: 0.96422,
    y: 1.00000,
    z: 0.82521,
};

impl XYZ {
    // pub const TO_RGB: Mat3 = Mat3::new(
    //     0.49000, 0.31000, 0.20000,
    //     0.17697, 0.81240, 0.01063,
    //     0.00000, 0.01000, 0.99000,
    // );

    pub const TO_RGB: Mat3 = Mat3::new(
        0.49000, 0.17697, 0.00000,
        0.31000, 0.81240, 0.01000,
        0.20000, 0.01063, 0.99000,
    );

    // pub const TO_RGB: Mat3 = Mat3::new(
    //     0.4124564, 0.2126729, 0.0193339,
    //     0.3575761, 0.7151522, 0.1191920,
    //     0.1804375, 0.0721750, 0.9503041,
    // );
}

// const RGB2XYZ: Mat3 = XYZ2RGB.invert().expect("RGB2XYZ must be invertible");

impl From<RGB> for XYZ {
    fn from(value: RGB) -> Self {
        let rgb2xyz = XYZ::TO_RGB.invert().expect("RGB2XYZ must be invertible");
        XYZ::from(rgb2xyz * Vec3::from(value))
    }
}