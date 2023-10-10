use three_d::Vec3;

pub struct Polyline {
    pub points: Vec<Vec3>,
    closed: bool,
}