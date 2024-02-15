use nalgebra::{Vector3, Vector4};

use crate::{
    gpu::ZBuffer,
    graphics::draw_triangle,
    shaders::{mesh_shader, MeshShader, PixelShader},
    types::{IndexedTriangle, RawPoint, Triangle, TriangleVertex},
};

pub struct Pipeline<const VSIN: usize, const PSIN: usize> {
    ms_output: Vec<Triangle<PSIN>>,

    screen_width: usize,
    screen_height: usize,
}

impl<const VSIN: usize, const PSIN: usize> Pipeline<VSIN, PSIN> {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            screen_width,
            screen_height,
            ms_output: Vec::new(),
        }
    }

    pub fn render_scene<
        MS: for<'a> MeshShader<
            (
                &'a [RawPoint<VSIN>],
                &'a [IndexedTriangle],
                &'a Vector3<f32>,
            ),
            PSIN,
        >,
        PS: PixelShader<PSIN>,
    >(
        &mut self,
        raw_vertices: &[RawPoint<VSIN>],
        raw_indices: &[IndexedTriangle],
        depth_buffer: &mut ZBuffer,
    ) {
        // Get our camera eye position
        let eye_position: Vector4<f32> =
            mesh_shader::get_projection_matrix().as_matrix() * Vector4::new(0.0, 0.0, 0.0, 1.0);
        let eye_position = eye_position.xyz();

        MS::run(
            (raw_vertices, raw_indices, &eye_position),
            &mut self.ms_output,
        );

        self.ms_output.drain(..).for_each(|mut triangle| {
            // Convert Tris to screen space
            triangle
                .vertices
                .iter_mut()
                .for_each(|vertex| to_ndc(vertex, self.screen_width, self.screen_height));
            draw_triangle::<PS, PSIN>(triangle, depth_buffer);
        });
    }
}

fn to_ndc<const PSIN: usize>(
    vertex: &mut TriangleVertex<PSIN>,
    screen_width: usize,
    screen_height: usize,
) {
    let w_inverse = vertex.position.w.recip();
    *vertex *= w_inverse;

    vertex.position.x = (vertex.position.x + 1.0) * (screen_width as f32 / 2.0);
    vertex.position.y = (-vertex.position.y + 1.0) * (screen_height as f32 / 2.0);

    vertex.position.w = w_inverse;
}
