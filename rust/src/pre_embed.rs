use cgmath::vec3;

use crate::mesh::CpuMesh;

/// Pre-embedded cylinder, where x indicates turn, y indicates height, and z indicates radius.
pub fn cylinder(horizontal_subdivisions: u32, vertical_subdivisions: u32, radial_subdivisions: u32) -> CpuMesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let stride = radial_subdivisions * 2 + vertical_subdivisions + 1;

    for hdiv in 0..horizontal_subdivisions+1 {
        let h = hdiv as f32 / horizontal_subdivisions as f32;
        positions.extend((0..vertical_subdivisions+1).map(|vdiv| vec3(h, vdiv as f32 / vertical_subdivisions as f32, 1.0)));

        for rdiv in 0..radial_subdivisions {
            let r = rdiv as f32 / radial_subdivisions as f32;
            positions.extend(&[
                vec3(h, 0.0, r),
                vec3(h, 1.0, r),
            ]);
        }
    }

    for hdiv in 0..horizontal_subdivisions {
        for vdiv in 0..vertical_subdivisions {
            let a = hdiv * stride + vdiv;
            let b = a + 1;
            let c = a + stride;
            let d = c + 1;
            indices.extend(&[
                a as u32, b as u32, c as u32,
                b as u32, d as u32, c as u32,
            ]);
        }
        for rdiv in 0..radial_subdivisions-1 {
            let a = hdiv * stride + vertical_subdivisions + 1 + rdiv * 2;
            let b = a + 2;
            let c = a + stride;
            let d = c + 2;
            let i = [
                a as u32, b as u32, c as u32,
                b as u32, d as u32, c as u32,
            ];
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
        indices.extend(&[
            ab, bb, cb,
            bb, db, cb,
            at, bt, ct,
            bt, dt, ct,
        ]);
    }

    CpuMesh::new(positions, Some(indices))
}