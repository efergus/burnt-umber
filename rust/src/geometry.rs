use three_d::{Vec3, degrees, Angle, vec3};


pub fn cylinder_mesh(subdivisions: i32) -> Vec<Vec3> {
    let mut positions: Vec<Vec3> = Vec::new();
    let top = Vec3::new(1.0, 0.0, 0.0);
    let bottom = Vec3::new(0.0, 0.0, 0.0);
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left_bottom = Vec3::new(0.0, angle.cos(), angle.sin());
        let left_top = Vec3::new(1.0, angle.cos(), angle.sin());
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right_bottom = Vec3::new(0.0, angle.cos(), angle.sin());
        let right_top = Vec3::new(1.0, angle.cos(), angle.sin());
        let left_turn = (left_turn * 255.0) as u8;
        let right_turn = (right_turn * 255.0) as u8;
        positions.push(top);
        positions.push(left_top);
        positions.push(right_top);

        positions.push(left_top);
        positions.push(left_bottom);
        positions.push(right_bottom);

        positions.push(left_top);
        positions.push(right_bottom);
        positions.push(right_top);

        positions.push(bottom);
        positions.push(right_bottom);
        positions.push(left_bottom);
    }
    positions
}

pub fn cone_mesh(subdivisions: i32) -> Vec<Vec3> {
    let mut positions: Vec<Vec3> = Vec::new();
    let top = Vec3::new(1.0, 0.0, 0.0);
    let bottom = Vec3::new(0.0, 0.0, 0.0);
    for i in 0..subdivisions {
        let left_turn = i as f32 / subdivisions as f32;
        let angle = degrees(left_turn * 360.0);
        let left = Vec3::new(0.0, angle.cos(), angle.sin());
        let right_turn = (i + 1) as f32 / subdivisions as f32;
        let angle = degrees(right_turn * 360.0);
        let right = Vec3::new(0.0, angle.cos(), angle.sin());
        positions.push(top);
        positions.push(left);
        positions.push(right);
        positions.push(left);
        positions.push(bottom);
        positions.push(right);
    }
    positions
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