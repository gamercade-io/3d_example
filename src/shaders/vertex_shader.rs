use std::mem::MaybeUninit;

use nalgebra::{Rotation3, SVector};

use crate::types::TriangleVertex;

// Processes verticies and places them into the output buffer
pub trait VertexShader<const VSIN: usize, const VSOUT: usize> {
    fn run(vertex: TriangleVertex<VSIN>) -> TriangleVertex<VSOUT>;
}

pub struct DefaultVertexShader;
static mut ROTATION: MaybeUninit<Rotation3<f32>> = MaybeUninit::uninit();
static mut CAMERA_POSITION: MaybeUninit<SVector<f32, 3>> = MaybeUninit::uninit();

impl DefaultVertexShader {
    pub fn bind_rotation(rotation: Rotation3<f32>) {
        unsafe {
            ROTATION.write(rotation);
        }
    }

    pub fn bind_camera_position(position: SVector<f32, 3>) {
        unsafe {
            CAMERA_POSITION.write(position);
        }
    }

    pub fn get_rot() -> Rotation3<f32> {
        unsafe { ROTATION.assume_init() }
    }

    pub fn get_camera_position() -> SVector<f32, 3> {
        unsafe { CAMERA_POSITION.assume_init() }
    }
}

impl<const VSINOUT: usize> VertexShader<VSINOUT, VSINOUT> for DefaultVertexShader {
    fn run(mut vertex: TriangleVertex<VSINOUT>) -> TriangleVertex<VSINOUT> {
        vertex.position = Self::get_rot() * vertex.position;
        vertex.position += Self::get_camera_position();
        vertex
    }
}
