use cgmath::vec3;
use three_d::Vec3;

use crate::mesh::CpuMesh;

pub fn plane(
    x_subdivisions: u32,
    y_subdivisions: u32,
    unit_x: Vec3,
    unit_y: Vec3,
    origin: Vec3,
) -> CpuMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let stride = y_subdivisions + 1;

    for hdiv in 0..x_subdivisions + 1 {
        let h = hdiv as f32 / x_subdivisions as f32;
        let base = origin + unit_x * h;
        positions.extend(
            (0..y_subdivisions + 1)
                .map(|vdiv| base + unit_y * (vdiv as f32 / y_subdivisions as f32)),
        );
    }

    for hdiv in 0..x_subdivisions {
        for vdiv in 0..y_subdivisions {
            let a = hdiv * stride + vdiv;
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            indices.extend(&[a as u32, b as u32, c as u32, b as u32, d as u32, c as u32]);
        }
    }

    CpuMesh::new(positions, Some(indices))
}

pub fn cube(
    horizontal_subdivisions: u32,
    vertical_subdivisions: u32,
    depth_subdivisions: u32,
) -> CpuMesh {
    let origin = vec3(0.0, 0.0, 0.0);
    let corner = vec3(1.0, 1.0, 1.0);
    let mut cube = plane(
        horizontal_subdivisions,
        vertical_subdivisions,
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        origin,
    );
    cube.extend(&plane(
        horizontal_subdivisions,
        depth_subdivisions,
        vec3(1.0, 0.0, 0.0),
        vec3(0.0, 0.0, 1.0),
        origin,
    ));
    cube.extend(&plane(
        depth_subdivisions,
        vertical_subdivisions,
        vec3(0.0, 0.0, 1.0),
        vec3(0.0, 1.0, 0.0),
        origin,
    ));
    cube.extend(&plane(
        horizontal_subdivisions,
        vertical_subdivisions,
        vec3(-1.0, 0.0, 0.0),
        vec3(0.0, -1.0, 0.0),
        corner,
    ));
    cube.extend(&plane(
        horizontal_subdivisions,
        depth_subdivisions,
        vec3(-1.0, 0.0, 0.0),
        vec3(0.0, 0.0, -1.0),
        corner,
    ));
    cube.extend(&plane(
        depth_subdivisions,
        vertical_subdivisions,
        vec3(0.0, 0.0, -1.0),
        vec3(0.0, -1.0, 0.0),
        corner,
    ));
    cube.face_away(vec3(0.5, 0.5, 0.5));
    cube
}

/// Pre-embedded cylinder, where x indicates turn, y indicates height, and z indicates radius.
pub fn _cylinder(
    horizontal_subdivisions: u32,
    vertical_subdivisions: u32,
    radial_subdivisions: u32,
) -> CpuMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let stride = radial_subdivisions * 2 + vertical_subdivisions + 1;

    for hdiv in 0..horizontal_subdivisions + 1 {
        let h = hdiv as f32 / horizontal_subdivisions as f32;
        positions.extend(
            (0..vertical_subdivisions + 1)
                .map(|vdiv| vec3(h, vdiv as f32 / vertical_subdivisions as f32, 1.0)),
        );

        for rdiv in 0..radial_subdivisions {
            let r = rdiv as f32 / radial_subdivisions as f32;
            positions.extend(&[vec3(h, 0.0, r), vec3(h, 1.0, r)]);
        }
    }

    for hdiv in 0..horizontal_subdivisions {
        for vdiv in 0..vertical_subdivisions {
            let a = hdiv * stride + vdiv;
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            indices.extend(&[a as u32, b as u32, c as u32, b as u32, d as u32, c as u32]);
        }
        for rdiv in 0..radial_subdivisions - 1 {
            let a = hdiv * stride + vertical_subdivisions + 1 + rdiv * 2;
            let b = a + 2;
            let c = a + stride;
            let d = c + 2;
            let i = [a as u32, b as u32, c as u32, b as u32, d as u32, c as u32];
            indices.extend(&i);
            indices.extend(i.map(|i| i + 1));
        }
        // Connect radial subdivisions to vertical subdivisions
        // bottom radial 1
        let ab = (hdiv + 1) * stride - 2;
        // bottom vertical 1
        let bb = hdiv * stride;
        // bottom radial 2
        let cb = (hdiv + 2) * stride - 2;
        // bottom vertical 2
        let db = (hdiv + 1) * stride;
        // top radial 1
        let at = ab + 1;
        // top vertical 1
        let bt = bb + vertical_subdivisions;
        // top radial 2
        let ct = cb + 1;
        // top vertical 2
        let dt = db + vertical_subdivisions;
        indices.extend(&[ab, bb, cb, bb, db, cb, at, bt, ct, bt, dt, ct]);
    }

    CpuMesh::new(positions, Some(indices))
}
