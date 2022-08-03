use nalgebra::{Vector2, Vector3};

use crate::types::{Color, IndexedTriangle, TriangleEdge};

pub const CUBE_EDGES: [TriangleEdge; 12] = [
    TriangleEdge(0, 1),
    TriangleEdge(1, 3),
    TriangleEdge(3, 2),
    TriangleEdge(2, 0),
    TriangleEdge(0, 4),
    TriangleEdge(1, 5),
    TriangleEdge(3, 7),
    TriangleEdge(2, 6),
    TriangleEdge(4, 5),
    TriangleEdge(5, 7),
    TriangleEdge(7, 6),
    TriangleEdge(6, 4),
];

pub const CUBE_indices: [IndexedTriangle; 12] = [
    IndexedTriangle(0, 2, 1),
    IndexedTriangle(2, 3, 1),
    IndexedTriangle(1, 3, 5),
    IndexedTriangle(3, 7, 5),
    IndexedTriangle(2, 6, 3),
    IndexedTriangle(3, 6, 7),
    IndexedTriangle(4, 5, 7),
    IndexedTriangle(4, 7, 6),
    IndexedTriangle(0, 4, 2),
    IndexedTriangle(2, 4, 6),
    IndexedTriangle(0, 1, 4),
    IndexedTriangle(1, 5, 4),
];

pub const CUBE_COLORS: [Vector3<f32>; 8] = [
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 1.0, 0.0),
    Vector3::new(0.0, 1.0, 1.0),
    Vector3::new(1.0, 0.0, 0.0),
    Vector3::new(1.0, 0.0, 1.0),
    Vector3::new(1.0, 1.0, 0.0),
    Vector3::new(1.0, 1.0, 1.0),
    Vector3::new(1.0, 0.0, 1.0),
];

pub const CUBE_UVS: [Vector2<f32>; 8] = [
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.0, 0.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 0.0),
    Vector2::new(0.0, 0.0),
];

pub const SIDE: f32 = 1.0;
pub fn cube(size: f32) -> [Vector3<f32>; 8] {
    let side = size * 0.5;
    [
        Vector3::new(-side, -side, -side),
        Vector3::new(side, -side, -side),
        Vector3::new(-side, side, -side),
        Vector3::new(side, side, -side),
        Vector3::new(-side, -side, side),
        Vector3::new(side, -side, side),
        Vector3::new(-side, side, side),
        Vector3::new(side, side, side),
    ]
}
