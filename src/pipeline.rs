use crate::{
    graphics::draw_triangle,
    shaders::{GeometryShader, PixelShader, VertexShader},
    types::{IndexedTriangle, Triangle, TriangleVertex},
};

pub struct Pipeline<const VSIN: usize, const GSIN: usize, const PSIN: usize> {
    gs_input: Vec<TriangleVertex<GSIN>>,
    triangle_buffer: Vec<Triangle<GSIN>>,
    ps_input: Vec<Triangle<PSIN>>,
    screen_width: u32,
    screen_height: u32,
}

impl<const VSIN: usize, const GSIN: usize, const PSIN: usize> Pipeline<VSIN, GSIN, PSIN> {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            screen_width,
            screen_height,
            gs_input: Vec::new(),
            triangle_buffer: Vec::new(),
            ps_input: Vec::new(),
        }
    }

    pub fn render_scene<
        VS: VertexShader<VSIN, GSIN>,
        GS: GeometryShader<GSIN, PSIN>,
        PS: PixelShader<PSIN>,
    >(
        &mut self,
        raw_vertices: &[TriangleVertex<VSIN>],
        raw_indices: &[IndexedTriangle],
    ) {
        // Clear the buffers
        self.gs_input.clear();

        // Process vertices by applying the Vertex Shader
        // to each vertex, and storing their output in gs_input
        self.gs_input
            .extend(raw_vertices.iter().map(|vertex| VS::run(*vertex)));

        // Assemble our triangles, using indices
        // and place them into the triangle buffer.
        self.triangle_buffer
            .extend(raw_indices.iter().map(|triangle_indices| {
                let a = self.gs_input[triangle_indices.0];
                let b = self.gs_input[triangle_indices.1];
                let c = self.gs_input[triangle_indices.2];

                Triangle {
                    vertices: [a, b, c],
                }
            }));

        // Run the geometry shader on each triangle and
        // store it in ps_input
        self.ps_input.extend(
            self.triangle_buffer
                .drain(..)
                .map(|triangle| GS::run(triangle)),
        );

        // Do backface Culling
        self.ps_input.retain(|triangle| {
            let a = triangle.vertices[0].position;
            let b = triangle.vertices[1].position;
            let c = triangle.vertices[2].position;

            let cross_result = (b - a).cross(&(c - a));
            let dot_result = cross_result.dot(&a);
            let cull_flag = dot_result > 0.0;
            !cull_flag
        });

        //Convert the verts into screen space
        self.ps_input.iter_mut().for_each(|triangle| {
            triangle
                .vertices
                .iter_mut()
                .for_each(|vertex| to_screen_space(vertex, self.screen_width, self.screen_height));
        });

        // Rasterize the triangles
        self.ps_input.drain(..).for_each(|triangle| {
            draw_triangle::<PS, PSIN>(triangle);
        });
    }
}

fn to_screen_space<const PSIN: usize>(
    vertex: &mut TriangleVertex<PSIN>,
    screen_width: u32,
    screen_height: u32,
) {
    let z_inverse = vertex.position.z.recip();

    *vertex *= z_inverse;

    vertex.position.x = (vertex.position.x + 1.0) * (screen_width as f32 / 2.0);
    vertex.position.y = (-vertex.position.y + 1.0) * (screen_height as f32 / 2.0);

    vertex.position.z = z_inverse;
}
