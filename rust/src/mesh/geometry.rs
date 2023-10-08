use cgmath::vec3;

use super::CpuMesh;

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
        // Left (x = 0)
        0, 1, 2, 1, 3, 2, // Right (x = 1)
        4, 6, 5, 5, 6, 7, // Up (y = 1)
        2, 3, 7, 2, 7, 6, // Down (y = 0)
        0, 4, 1, 1, 4, 5, // Back (z = 1)
        0, 2, 4, 2, 6, 4, // Front (z = 0)
        1, 5, 3, 3, 5, 7,
    ];

    let mut cube = CpuMesh { positions, indices };
    cube.face_away(vec3(0.5, 0.5, 0.5));
    cube
}
