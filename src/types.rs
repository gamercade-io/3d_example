use std::ops::{AddAssign, Div, Mul, Sub};

use nalgebra::Vector3;

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

pub struct Triangle<T> {
    pub verticies: [TriangleInner<T>; 3],
}

#[derive(Clone, Copy)]
pub struct TriangleInner<T> {
    pub position: Vector3<f32>,
    pub parameters: T,
}

impl<T> Sub for TriangleInner<T>
where
    T: Sub + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        TriangleInner {
            position: self.position - rhs.position,
            parameters: self.parameters - rhs.parameters,
        }
    }
}

impl<T> Div<f32> for TriangleInner<T>
where
    T: Div<f32> + Div<f32, Output = T>,
{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        TriangleInner {
            position: self.position / rhs,
            parameters: self.parameters / rhs,
        }
    }
}

impl<T> Mul<f32> for TriangleInner<T>
where
    T: Mul<f32> + Mul<f32, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        TriangleInner {
            position: self.position * rhs,
            parameters: self.parameters * rhs,
        }
    }
}

impl<T> AddAssign for TriangleInner<T>
where
    T: AddAssign,
{
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
