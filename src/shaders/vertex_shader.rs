use std::mem::MaybeUninit;

use nalgebra::{Perspective3, Transform3};

use crate::types::{RawPoint, TriangleVertex};
static mut MODEL: MaybeUninit<Transform3<f32>> = MaybeUninit::uninit();
static mut VIEW: MaybeUninit<Transform3<f32>> = MaybeUninit::uninit();
static mut PROJECTION: MaybeUninit<Perspective3<f32>> = MaybeUninit::uninit();

// Processes verticies and places them into the output buffer
pub trait VertexShader<const VSIN: usize, const VSOUT: usize> {
    fn run(vertex: RawPoint<VSIN>) -> TriangleVertex<VSOUT>;
}

pub fn get_projection_matrix() -> Perspective3<f32> {
    unsafe { PROJECTION.assume_init() }
}

pub fn bind_view_matrix(view: Transform3<f32>) {
    unsafe {
        VIEW.write(view);
    }
}

pub fn get_model_matrix() -> Transform3<f32> {
    unsafe { MODEL.assume_init() }
}

pub fn bind_model_matrix(model: Transform3<f32>) {
    unsafe {
        MODEL.write(model);
    }
}

pub fn init_projection(screen_width: usize, screen_height: usize) {
    // 103 for 16:9
    let fov = 103f32.to_radians();
    let close = 1.0;
    let far = 1000.0;

    unsafe {
        PROJECTION.write(Perspective3::new(
            screen_width as f32 / screen_height as f32,
            fov,
            close,
            far,
        ));
    }
}

pub fn get_view_matrix() -> Transform3<f32> {
    unsafe { VIEW.assume_init() }
}

pub struct DefaultVertexShader;

impl<const VSINOUT: usize> VertexShader<VSINOUT, VSINOUT> for DefaultVertexShader {
    fn run(vertex: RawPoint<VSINOUT>) -> TriangleVertex<VSINOUT> {
        let model = get_model_matrix();
        let view = get_view_matrix();
        let projection = get_projection_matrix();

        let mvp = projection.as_matrix() * (view * model).to_homogeneous();

        let position = mvp.transform_point(&vertex.position);

        TriangleVertex {
            position: position.into(),
            parameters: vertex.parameters,
        }
    }
}
