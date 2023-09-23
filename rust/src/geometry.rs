use three_d::{degrees, vec2, vec3, Angle, Vec2, Vec3};

fn polar(turn: f32) -> Vec2 {
    let angle = degrees(turn * 360.0);
    vec2(angle.cos(), angle.sin())
}

pub fn polar_generator<F: Fn(Vec2, Vec2) -> Vec<Vec3>>(
    subdivisions: u32,
    start: f32,
    end: f32,
    func: F,
) -> Vec<Vec3> {
    let mut positions: Vec<Vec3> = Vec::new();
    let diff = end - start;
    for i in 0..subdivisions {
        let left_turn = (i as f32 / subdivisions as f32) * diff + start;
        let left = polar(left_turn);
        let right_turn = ((i + 1) as f32 / subdivisions as f32) * diff + start;
        let right = polar(right_turn);
        positions.extend(&func(left, right));
    }
    positions
}

pub fn unwrap_mesh(mesh: &Vec<Vec3>) -> Vec<Vec3> {
    mesh.iter()
        .map(|pos| {
            let flat = vec2(pos.x, pos.z);
            let mut angle = -flat.y.atan2(flat.x) / std::f32::consts::PI / 2.0;
            if angle < 0.0 {
                angle += 1.0;
            }
            vec3(angle, pos.y, 0.0)
        })
        .collect()
}

pub fn tube_mesh(subdivisions: u32) -> Vec<Vec3> {
    polar_generator(subdivisions, 0.0, 1.0, |left, right| {
        let left_top = vec3(left.x, 1.0, -left.y);
        let right_top = vec3(right.x, 1.0, -right.y);
        let left_bottom = vec3(left.x, 0.0, -left.y);
        let right_bottom = vec3(right.x, 0.0, -right.y);
        vec![
            left_top,
            left_bottom,
            right_bottom,
            left_top,
            right_bottom,
            right_top,
        ]
    })
}

pub fn cylinder_mesh(subdivisions: u32) -> Vec<Vec3> {
    polar_generator(subdivisions, 0.0, 1.0, |left, right| {
        let top = vec3(0.0, 1.0, 0.0);
        let left_top = vec3(left.x, 1.0, left.y);
        let right_top = vec3(right.x, 1.0, right.y);
        let bottom = vec3(0.0, 0.0, 0.0);
        let left_bottom = vec3(left.x, 0.0, left.y);
        let right_bottom = vec3(right.x, 0.0, right.y);
        vec![
            left_top,
            top,
            right_top,
            left_bottom,
            left_top,
            right_bottom,
            right_bottom,
            left_top,
            right_top,
            right_bottom,
            bottom,
            left_bottom,
        ]
    })
}

pub fn _cone_mesh(subdivisions: u32) -> Vec<Vec3> {
    polar_generator(subdivisions, 0.0, 1.0, |left, right| {
        let top = vec3(0.0, 1.0, 0.0);
        let bottom = vec3(0.0, 0.0, 0.0);
        let left = vec3(left.x, 1.0, left.y);
        let right = vec3(right.x, 1.0, right.y);
        vec![top, left, right, left, bottom, right]
    })
}

pub fn quad_mesh() -> Vec<Vec3> {
    return vec![
        vec3(0.0, 0.0, 0.0),
        vec3(1.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(1.0, 0.0, 0.0),
        vec3(1.0, 1.0, 0.0),
    ];
}

// fn sphere(subdivisions: u32) -> Vec<Vec3> {

// }
