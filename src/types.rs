use nalgebra::Vector2;

pub struct TriangleEdge(pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct IndexedTriangle(pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_index(self) -> usize {
        let r = self.r as usize / 17;
        let g = self.g as usize / 17;
        let b = self.b as usize / 17;

        (r * 256) + (g * 16) + b
    }
}

pub struct Triangle {
    pub verticies: [Vector2<f32>; 3],
}
