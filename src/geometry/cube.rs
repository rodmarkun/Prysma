use crate::math::Vec3;

pub const VERTICES: [Vec3; 8] = [
    Vec3 { x: -1.0, y: -1.0, z: -1.0 },  // 0: front bottom left
    Vec3 { x:  1.0, y: -1.0, z: -1.0 },  // 1: front bottom right
    Vec3 { x:  1.0, y:  1.0, z: -1.0 },  // 2: front top right
    Vec3 { x: -1.0, y:  1.0, z: -1.0 },  // 3: front top left
    Vec3 { x: -1.0, y: -1.0, z:  1.0 },  // 4: back bottom left
    Vec3 { x:  1.0, y: -1.0, z:  1.0 },  // 5: back bottom right
    Vec3 { x:  1.0, y:  1.0, z:  1.0 },  // 6: back top right
    Vec3 { x: -1.0, y:  1.0, z:  1.0 },  // 7: back top left
];

pub const EDGES: [(usize, usize); 12] = [
    (0, 1), (1, 2), (2, 3), (3, 0),
    (4, 5), (5, 6), (6, 7), (7, 4),
    (0, 4), (1, 5), (2, 6), (3, 7),
];

// Each face is 2 triangles, vertices in counter-clockwise order (for backface culling)
// (v0, v1, v2) - counter-clockwise when viewed from outside
pub const TRIANGLES: [(usize, usize, usize); 12] = [
    // Front face (z = -1)
    (0, 1, 2), (0, 2, 3),
    // Back face (z = 1)
    (5, 4, 7), (5, 7, 6),
    // Left face (x = -1)
    (4, 0, 3), (4, 3, 7),
    // Right face (x = 1)
    (1, 5, 6), (1, 6, 2),
    // Top face (y = 1)
    (3, 2, 6), (3, 6, 7),
    // Bottom face (y = -1)
    (4, 5, 1), (4, 1, 0),
];
