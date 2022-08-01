use nalgebra::Vector3;

use crate::types::Color;

pub trait FragmentShader<FS> {
    fn frag(shader_params: FS) -> Color;
}

pub struct ColorBlend;

impl FragmentShader<Vector3<f32>> for ColorBlend {
    fn frag(shader_params: Vector3<f32>) -> Color {
        Color {
            r: (shader_params.x * 255.0) as u8,
            g: (shader_params.y * 255.0) as u8,
            b: (shader_params.z * 255.0) as u8,
        }
    }
}
