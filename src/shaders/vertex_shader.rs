use std::mem::MaybeUninit;

use nalgebra::Transform3;

use crate::types::{RawPoint, TriangleVertex};

// Processes verticies and places them into the output buffer
pub trait VertexShader<const VSIN: usize, const VSOUT: usize> {
    fn run(vertex: RawPoint<VSIN>) -> TriangleVertex<VSOUT>;
}

pub struct DefaultVertexShader;
static mut TRANSFORM: MaybeUninit<Transform3<f32>> = MaybeUninit::uninit();

impl DefaultVertexShader {
    pub fn bind_transform(transform: Transform3<f32>) {
        unsafe {
            TRANSFORM.write(transform);
        }
    }

    pub fn get_transform() -> Transform3<f32> {
        unsafe { TRANSFORM.assume_init() }
    }
}

impl<const VSINOUT: usize> VertexShader<VSINOUT, VSINOUT> for DefaultVertexShader {
    fn run(vertex: RawPoint<VSINOUT>) -> TriangleVertex<VSINOUT> {
        let position = Self::get_transform().transform_point(&vertex.position);

        TriangleVertex {
            position: position.into(),
            parameters: vertex.parameters,
        }
    }
}
