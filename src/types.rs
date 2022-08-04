use std::ops::{AddAssign, Div, Mul, MulAssign, Sub};

use gamercade_rs::GraphicsParameters;
use nalgebra::{Point3, SVector, Vector4};

pub struct TriangleEdge(pub usize, pub usize);

pub struct IndexedTriangle(pub usize, pub usize, pub usize);

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// static mut COLOR_LOOKUP: MaybeUninit<[i32; 32 * 32 * 16]> = MaybeUninit::uninit();

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_graphics_params(self) -> GraphicsParameters {
        let a_level = self.r / 8;
        let g_level = self.g / 8;
        let b_level = self.b / 16;

        let g_palette = g_level / 4;
        let g_color = (g_level % 4) * 16;

        let r_palette = a_level * 8;

        GraphicsParameters::default()
            .palette_index(r_palette + g_palette as u8)
            .color_index(g_color + b_level as u8)
    }
}

pub struct Triangle<const IN: usize> {
    pub vertices: [TriangleVertex<IN>; 3],
}

#[derive(Copy, Clone)]
pub struct RawPoint<const IN: usize> {
    pub position: Point3<f32>,
    pub parameters: SVector<f32, IN>,
}

#[derive(Clone, Copy)]
pub struct TriangleVertex<const IN: usize> {
    pub position: Vector4<f32>,
    pub parameters: SVector<f32, IN>,
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
