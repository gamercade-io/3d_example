use std::mem::MaybeUninit;

use gamercade_rs::api::text::console_log;
use nalgebra::{Perspective3, Transform3, Vector3, Vector4};

use crate::types::{IndexedTriangle, RawPoint, Triangle, TriangleVertex};

// Receives MSIN input parameters and output buffer of Triangles
pub trait MeshShader<MSIN, const PSIN: usize> {
    fn run(inputs: MSIN, output: &mut Vec<Triangle<PSIN>>);
}

pub struct DefaultMeshShader<const PSIN: usize>;

static mut MODEL: MaybeUninit<Transform3<f32>> = MaybeUninit::uninit();
static mut VIEW: MaybeUninit<Transform3<f32>> = MaybeUninit::uninit();
static mut PROJECTION: MaybeUninit<Perspective3<f32>> = MaybeUninit::uninit();

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
    let aspect_ratio = screen_width as f32 / screen_height as f32;
    let hfov = 103f32.to_radians();
    let vfov = 2.0 * ((hfov / 2.0).tan() * aspect_ratio.recip()).atan();
    console_log(&format!("vertical fov: {}", vfov.to_degrees()));
    let close = 1.0;
    let far = 1000.0;

    unsafe {
        PROJECTION.write(Perspective3::new(aspect_ratio, vfov, close, far));
    }
}

pub fn get_view_matrix() -> Transform3<f32> {
    unsafe { VIEW.assume_init() }
}

impl<const PSIN: usize> MeshShader<(&[RawPoint<PSIN>], &[IndexedTriangle], &Vector3<f32>), PSIN>
    for DefaultMeshShader<PSIN>
{
    fn run(
        (vertices, indices, eye_position): (&[RawPoint<PSIN>], &[IndexedTriangle], &Vector3<f32>),
        output: &mut Vec<Triangle<PSIN>>,
    ) {
        indices.iter().for_each(|indices| {
            // Modify vertices by position / camera things
            let vertex_a = &vertices[indices.0];
            let vertex_b = &vertices[indices.1];
            let vertex_c = &vertices[indices.2];

            // Handle backface Culling
            let va_pos = vertex_pos(vertex_a);
            let vb_pos = vertex_pos(vertex_b);
            let vc_pos = vertex_pos(vertex_c);

            let a = va_pos.xyz();
            let b = vb_pos.xyz();
            let c = vc_pos.xyz();

            let cross_result = (b - a).cross(&(c - a));
            let dot_compare = a - eye_position;
            let dot_result = cross_result.dot(&dot_compare);
            if dot_result >= 0.0 {
                return;
            }
            // End backface Culling

            //TODO Clipping
            output.push(Triangle {
                vertices: [
                    TriangleVertex {
                        position: va_pos,
                        parameters: vertex_a.parameters,
                    },
                    TriangleVertex {
                        position: vb_pos,
                        parameters: vertex_b.parameters,
                    },
                    TriangleVertex {
                        position: vc_pos,
                        parameters: vertex_c.parameters,
                    },
                ],
            })
        });
    }
}

fn vertex_pos<const PSIN: usize>(vertex: &RawPoint<PSIN>) -> Vector4<f32> {
    let position: Vector4<f32> = vertex.position.into();
    let model = get_model_matrix();
    let view = get_view_matrix();
    let projection = get_projection_matrix();

    let mvp = projection.as_matrix() * (model * view).to_homogeneous();
    let position = mvp * position;
    position
}

enum TriangleClipResult {
    AllInside,
    OneInside,
    TwoInside,
    AllOutside,
}
