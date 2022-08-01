use std::marker::PhantomData;

use nalgebra::{Rotation3, SVector, Vector3};

use crate::{
    fragment_shader::FragmentShader,
    graphics::draw_triangle,
    types::{Color, IndexedTriangle, Triangle, TriangleEdge, TriangleVertex},
    GameState,
};

#[derive(Default)]
pub struct Gpu<FS: FragmentShader<D>, const D: usize> {
    raw_vertex_buffer: Vec<Vector3<f32>>,
    triangle_buffer: Vec<Triangle<D>>,
    fragment_shader: PhantomData<FS>,
}

impl<FS: FragmentShader<D>, const D: usize> Gpu<FS, D> {
    // Clears our buffers to use next frame
    pub fn clear(&mut self) {
        self.raw_vertex_buffer.clear();
    }

    // Prepares the scene for drawing, including taking from the game state
    // the geometry, the index buffer, and converting them to screenspace.
    pub fn render_scene(&mut self, game_state: &GameState) {
        self.process_verticies(game_state);
        self.assemble_triangles(game_state);
        self.process_triangles(game_state);
        self.post_process_triangles(game_state);
        self.render(game_state);
    }

    // Prepares the verticies by appling any rotations and transformations
    fn process_verticies(&mut self, game_state: &GameState) {
        let rot = Rotation3::from_euler_angles(game_state.roll, game_state.pitch, game_state.yaw);

        // Transform our geometry and push it into the gpu
        self.raw_vertex_buffer
            .extend(game_state.vertex_data.iter().map(|vertex| {
                let mut vertex = rot * vertex;
                vertex += game_state.camera_position;

                //TODO: Add vertex shader
                vertex
            }));
    }

    // Build the index list, will filter out backfacing triangles
    fn assemble_triangles(&mut self, game_state: &GameState) {
        self.triangle_buffer
            .extend(game_state.index_data.iter().filter_map(|indexed_triangle| {
                let a = self.raw_vertex_buffer[indexed_triangle.0];
                let b = self.raw_vertex_buffer[indexed_triangle.1];
                let c = self.raw_vertex_buffer[indexed_triangle.2];

                let cross_result = (b - a).cross(&(c - a));
                let dot_result = cross_result.dot(&a);
                let cull_flag = dot_result > 0.0;
                if cull_flag {
                    None
                } else {
                    // This triangle is valid, so we also want to enqueue the extra vertex parameters
                    let a_params = &game_state.vertex_shader_inputs
                        [indexed_triangle.0 * D..(indexed_triangle.0 * D) + D];
                    let b_params = &game_state.vertex_shader_inputs
                        [indexed_triangle.1 * D..(indexed_triangle.1 * D) + D];
                    let c_params = &game_state.vertex_shader_inputs
                        [indexed_triangle.2 * D..(indexed_triangle.2 * D) + D];

                    let verticies = [
                        TriangleVertex {
                            position: a,
                            parameters: SVector::<f32, D>::from_column_slice(a_params),
                        },
                        TriangleVertex {
                            position: b,
                            parameters: SVector::<f32, D>::from_column_slice(b_params),
                        },
                        TriangleVertex {
                            position: c,
                            parameters: SVector::<f32, D>::from_column_slice(c_params),
                        },
                    ];

                    Some(Triangle { verticies })
                }
            }));
    }

    fn process_triangles(&mut self, game_state: &GameState) {
        // Convert the verticies to screen space
        self.triangle_buffer.iter_mut().for_each(|triangle| {
            triangle
                .verticies
                .iter_mut()
                .for_each(|vertex| to_screen_space(vertex, game_state))
        });
    }

    fn post_process_triangles(&mut self, _game_state: &GameState) {
        //TODO:
    }

    /// Actually go and render the geometry
    fn render(&mut self, game_state: &GameState) {
        // Render our geometry
        self.triangle_buffer.drain(..).for_each(|triangle| {
            draw_triangle::<FS, D>(triangle, game_state);
        });
    }
}

fn to_screen_space<const D: usize>(vertex: &mut TriangleVertex<D>, game_state: &GameState) {
    let z_inverse = vertex.position.z.recip();

    *vertex *= z_inverse;

    vertex.position.x = (vertex.position.x + 1.0) * (game_state.screen_width as f32 / 2.0);
    vertex.position.y = (-vertex.position.y + 1.0) * (game_state.screen_height as f32 / 2.0);

    vertex.position.z = z_inverse;
}

pub const CUBE_EDGES: [TriangleEdge; 12] = [
    TriangleEdge(0, 1),
    TriangleEdge(1, 3),
    TriangleEdge(3, 2),
    TriangleEdge(2, 0),
    TriangleEdge(0, 4),
    TriangleEdge(1, 5),
    TriangleEdge(3, 7),
    TriangleEdge(2, 6),
    TriangleEdge(4, 5),
    TriangleEdge(5, 7),
    TriangleEdge(7, 6),
    TriangleEdge(6, 4),
];

pub const CUBE_INDICIES: [IndexedTriangle; 12] = [
    IndexedTriangle(0, 2, 1),
    IndexedTriangle(2, 3, 1),
    IndexedTriangle(1, 3, 5),
    IndexedTriangle(3, 7, 5),
    IndexedTriangle(2, 6, 3),
    IndexedTriangle(3, 6, 7),
    IndexedTriangle(4, 5, 7),
    IndexedTriangle(4, 7, 6),
    IndexedTriangle(0, 4, 2),
    IndexedTriangle(2, 4, 6),
    IndexedTriangle(0, 1, 4),
    IndexedTriangle(1, 5, 4),
];

pub const CUBE_COLORS: [Color; 8] = [
    Color::new(0, 0, 0xFF),
    Color::new(0, 0xFF, 0),
    Color::new(0, 0xFF, 0xFF),
    Color::new(0xFF, 0, 0),
    Color::new(0xFF, 0, 0xFF),
    Color::new(0xFF, 0xFF, 0),
    Color::new(0xFF, 0xFF, 0xFF),
    Color::new(0xFF, 0, 0xFF),
];

pub const CUBE_UVS: [[f32; 2]; 8] = [
    [0.0, 1.0],
    [1.0, 1.0],
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [0.0, 0.0],
];
