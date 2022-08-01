use std::ops::{AddAssign, Div, Mul, MulAssign, Sub};

use nalgebra::{SVector, Vector3};

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

    pub fn to_4bit_index(self) -> usize {
        let r = self.r as usize / 17;
        let g = self.g as usize / 17;
        let b = self.b as usize / 17;

        (r * 256) + (g * 16) + b
    }

    pub fn to_554_index(self) -> usize {
        let r = self.r as usize / 8;
        let g = self.g as usize / 8;
        let b = self.b as usize / 17;

        (r * 512) + (g * 16) + b
    }
}

pub struct Triangle<const D: usize> {
    pub verticies: [TriangleVertex<D>; 3],
}

#[derive(Clone, Copy)]
pub struct TriangleVertex<const D: usize> {
    pub position: Vector3<f32>,
    pub parameters: SVector<f32, D>,
}

impl<const D: usize> Sub for TriangleVertex<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        TriangleVertex {
            position: self.position - rhs.position,
            parameters: self.parameters - rhs.parameters,
        }
    }
}

impl<const D: usize> Div<f32> for TriangleVertex<D> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        TriangleVertex {
            position: self.position / rhs,
            parameters: self.parameters / rhs,
        }
    }
}

impl<const D: usize> Mul<f32> for TriangleVertex<D> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        TriangleVertex {
            position: self.position * rhs,
            parameters: self.parameters * rhs,
        }
    }
}

impl<const D: usize> MulAssign<f32> for TriangleVertex<D> {
    fn mul_assign(&mut self, rhs: f32) {
        self.position *= rhs;
        self.parameters *= rhs;
    }
}

impl<const D: usize> AddAssign for TriangleVertex<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.position += rhs.position;
        self.parameters += rhs.parameters;
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_554_color() {
        let max = Color {
            r: 255,
            g: 255,
            b: 255,
        };

        assert_eq!(max.to_554_index(), 2 ^ 14 - 1);
    }

    #[test]
    fn test_4bit_color() {
        let max = Color {
            r: 255,
            g: 255,
            b: 255,
        };

        assert_eq!(max.to_4bit_index(), 2 ^ 16 - 1);
    }
}
