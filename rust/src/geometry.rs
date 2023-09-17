use three_d::{degrees, vec3, Angle, Vec2, Vec3, Deg, vec2};

pub fn polar_generator<F: Fn(Deg<f32>, Vec2, Vec2)->Vec<Vec3>>(subdivisions: u32, func: F) -> Vec<Vec3> {
    let mut positions: Vec<Vec3> = Vec::new();
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left = vec2(angle.cos(), angle.sin());
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right = vec2(angle.cos(), angle.sin());
        positions.extend(&func(angle, left, right));
    }
    positions
}

pub fn cylinder_mesh(subdivisions: u32) -> Vec<Vec3> {
    polar_generator(subdivisions, |_, left, right| {
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
            left_bottom
        ]
    })
}

pub fn _cone_mesh(subdivisions: u32) -> Vec<Vec3> {
    // let mut positions: Vec<Vec3> = Vec::new();
    // let top = Vec3::new(1.0, 0.0, 0.0);
    // let bottom = Vec3::new(0.0, 0.0, 0.0);
    // for i in 0..subdivisions {
    //     let left_turn = i as f32 / subdivisions as f32;
    //     let angle = degrees(left_turn * 360.0);
    //     let left = Vec3::new(0.0, angle.cos(), angle.sin());
    //     let right_turn = (i + 1) as f32 / subdivisions as f32;
    //     let angle = degrees(right_turn * 360.0);
    //     let right = Vec3::new(0.0, angle.cos(), angle.sin());
    //     positions.push(top);
    //     positions.push(left);
    //     positions.push(right);
    //     positions.push(left);
    //     positions.push(bottom);
    //     positions.push(right);
    // }
    polar_generator(subdivisions, |_, left, right| {
        let top = vec3(0.0, 1.0, 0.0);
        let bottom = vec3(0.0, 0.0, 0.0);
        let left = vec3(left.x, 1.0, left.y);
        let right = vec3(right.x, 1.0, right.y);
        vec![
            top,
            left,
            right,

            left,
            bottom,
            right,
        ]
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
