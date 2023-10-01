pub mod oklab;

use derive_more::{Add, Sub};

use three_d::{vec3, Vec3};
use wasm_bindgen::prelude::wasm_bindgen;

fn min(v: Vec3) -> f32 {
    v.x.min(v.y).min(v.z)
}

fn max(v: Vec3) -> f32 {
    v.x.max(v.y).max(v.z)
}

// Generate code like the following:
/*
impl From<Vec3> for RGB {
    fn from(v: Vec3) -> Self {
        RGB { r: v.x, g: v.y, b: v.z }
    }
}
 */
// #[proc_macro]
// pub fn into_vec3(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = input.ident;
//     let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
//     let fields = match input.data {
//         Data::Struct(DataStruct {
//             fields: Fields::Named(FieldsNamed { named, .. }),
//             ..
//         }) => named,
//         _ => panic!("Only named structs are supported"),
//     };
//     let mut field_names = Vec::new();
//     let mut field_values = Vec::new();
//     for field in fields {
//         let field_name = field.ident.as_ref().unwrap();
//         field_names.push(field_name);
//         field_values.push(quote! { #field_name: v.#field_name });
//     }
//     let expanded = quote! {
//         impl #impl_generics From<Vec3> for #name #ty_generics #where_clause {
//             fn from(v: Vec3) -> Self {
//                 #name { #(#field_values),* }
//             }
//         }
//     };
//     TokenStream::from(expanded)
// }


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Add, Sub)]
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

pub mod cie {
    use three_d::{Mat3, SquareMatrix, Vec3, vec3};

    pub struct XYY {
        x: f32,
        y: f32,
        #[allow(non_snake_case)]
        Y: f32,
    }

    pub struct XYZ {
        x: f32,
        y: f32,
        z: f32,
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

    const XYZ2RGB: Mat3 = Mat3::new(
        0.49000, 0.31000, 0.20000,
        0.17697, 0.81240, 0.01063,
        0.00000, 0.01000, 0.99000,
    );

    // const RGB2XYZ: Mat3 = XYZ2RGB.invert().expect("RGB2XYZ must be invertible");

    impl From<RGB> for XYZ {
        fn from(value: RGB) -> Self {
            let rgb2xyz = XYZ2RGB.invert().expect("RGB2XYZ must be invertible");
            XYZ::from(rgb2xyz * Vec3::from(value))
        }
    }
}
