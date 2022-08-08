use nalgebra::SVector;

use crate::{
    image::{self, IMAGE_HEIGHT, IMAGE_WIDTH},
    types::Color,
};

// Receives PSIN input parameters and outputs a pixel color
pub trait PixelShader<const PSIN: usize> {
    fn run(params: SVector<f32, PSIN>) -> Color;
}

pub struct ColorBlend;

impl PixelShader<3> for ColorBlend {
    fn run(shader_params: SVector<f32, 3>) -> Color {
        Color {
            r: (shader_params.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (shader_params.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (shader_params.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

pub struct Textured;

impl Textured {
    fn sample_2d(u: f32, v: f32) -> Color {
        let u = (u * (IMAGE_WIDTH - 1) as f32) as usize;
        let v = (v * (IMAGE_HEIGHT - 1) as f32) as usize;

        let u = u.clamp(0, IMAGE_WIDTH - 1);
        let v = v.clamp(0, IMAGE_HEIGHT - 1);

        image::get_image()[(v * IMAGE_WIDTH) + u]
    }
}

impl PixelShader<2> for Textured {
    fn run(shader_params: SVector<f32, 2>) -> Color {
        Self::sample_2d(shader_params.x, shader_params.y)
    }
}
