use cgmath::vec3;

use super::{CpuMesh, polyline::Polyline};

pub fn cube() -> CpuMesh {
    let mut positions = Vec::new();
    // z, then y, then x
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                positions.push(vec3(i as f32, j as f32, k as f32));
            }
        }
    }
    let indices = vec![
        0, 1, 2, 1, 3, 2, // Left (x = 0)
        4, 6, 5, 5, 6, 7, // Right (x = 1)
        2, 3, 7, 2, 7, 6, // Up (y = 1)
        0, 4, 1, 1, 4, 5, // Down (y = 0)
        0, 2, 4, 2, 6, 4, // Back (z = 1)
        1, 5, 3, 3, 5, 7, // Front (z = 0)
    ];

    let mut cube = CpuMesh { positions, indices };
    cube.face_away(vec3(0.5, 0.5, 0.5));
    cube
}

pub fn lathe(polyline: &Polyline, subdivisions: u32, arc: f32) -> CpuMesh {
    let mut positions = polyline.points.clone();
    let mut indices = Vec::new();
    let points = polyline.points.len() as u32;
    
    for i in 1..subdivisions {
        let prev = (i - 1) * points;
        let index = i * points;
        let angle = arc * i as f32 / subdivisions as f32;
        let cos = angle.cos();
        let sin = angle.sin();
        positions.extend(polyline.points.iter().map(|point| vec3(point.x * cos, point.y, point.x * sin)));
        for j in 0..points - 1 {
            let c = j + 0 + prev;
            let d = j + 1 + prev;
            let a = j + 0 + index;
            let b = j + 1 + index;
            indices.extend(vec![a as u32, b as u32, c as u32, a as u32, c as u32, d as u32]);
        }
    }

    CpuMesh::new(positions, Some(indices))
}