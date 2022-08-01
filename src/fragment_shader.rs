use nalgebra::SVector;

use crate::{
    image::{self, IMAGE_HEIGHT, IMAGE_WIDTH},
    types::Color,
};

pub trait FragmentShader<const D: usize> {
    fn frag(shader_params: SVector<f32, D>) -> Color;
}

#[derive(Default)]
pub struct ColorBlend;

impl FragmentShader<3> for ColorBlend {
    fn frag(shader_params: SVector<f32, 3>) -> Color {
        Color {
            r: (shader_params.x * 255.0) as u8,
            g: (shader_params.y * 255.0) as u8,
            b: (shader_params.z * 255.0) as u8,
        }
    }
}

#[derive(Default)]
pub struct Textured;

impl Textured {
    fn sample_2d(u: f32, v: f32) -> Color {
        let u = (u * (IMAGE_WIDTH - 1) as f32) as usize;
        let v = (v * (IMAGE_HEIGHT - 1) as f32) as usize;

        let u = u.clamp(0, IMAGE_WIDTH);
        let v = v.clamp(0, IMAGE_HEIGHT);

        image::get_image()[(v * IMAGE_WIDTH) + u]
    }
}

impl FragmentShader<2> for Textured {
    fn frag(shader_params: SVector<f32, 2>) -> Color {
        Self::sample_2d(shader_params.x, shader_params.y)
    }
}
