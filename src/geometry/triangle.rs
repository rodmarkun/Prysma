use crate::math::Vec3;

pub const VERTICES: [Vec3; 5] = [
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }, // 0: front bottom left
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    }, // 1: front bottom right
    Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    }, // 2: back bottom left
    Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    }, // 3: back bottom right
    Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    }, // 4: top
];
pub const EDGES: [(usize, usize); 8] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
];
