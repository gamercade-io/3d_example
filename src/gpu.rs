use nalgebra::{Rotation3, Vector3};

use crate::{
    graphics::draw_triangle,
    types::{Color, IndexedTriangle, Triangle, TriangleEdge},
    GameState,
};

#[derive(Default)]
pub struct Gpu {
    vertex_buffer: Vec<Vector3<f32>>,
    index_buffer: Vec<IndexedTriangle>,
    cull_flags: Vec<bool>,
}

impl Gpu {
    // Clears our buffers to use next frame
    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
        self.cull_flags.clear();
    }

    // Prepares the scene for drawing, including taking from the game state
    // the geometry, the index buffer, and converting them to screenspace.
    pub fn prepare_scene(&mut self, game_state: &GameState) {
        self.prepare_geometry(game_state);
        self.prepare_index_buffer(game_state);
        self.vertex_buffer_to_screen_space(game_state);
    }

    fn prepare_geometry(&mut self, game_state: &GameState) {
        let rot = Rotation3::from_euler_angles(game_state.roll, game_state.pitch, game_state.yaw);

        // Transform our geometry and push it into the gpu
        self.vertex_buffer
            .extend(game_state.vertex_data.iter().map(|vertex| {
                let mut vertex = rot * vertex;
                vertex.z += game_state.offset_z;
                vertex
            }));
    }

    fn prepare_index_buffer(&mut self, game_state: &GameState) {
        // Build the index list, and check for backfacing tris
        self.index_buffer
            .extend(game_state.index_data.iter().map(|indicies| {
                let verts = [
                    self.vertex_buffer[indicies.0],
                    self.vertex_buffer[indicies.1],
                    self.vertex_buffer[indicies.2],
                ];

                let cross_result = (verts[1] - verts[0]).cross(&(verts[2] - verts[0]));
                let dot_result = cross_result.dot(&verts[0]);
                let cull_flag = dot_result > 0.0;
                self.cull_flags.push(cull_flag);

                IndexedTriangle(indicies.0, indicies.1, indicies.2)
            }));
    }

    fn vertex_buffer_to_screen_space(&mut self, game_state: &GameState) {
        // Convert the verticies to screen space
        self.vertex_buffer.iter_mut().for_each(|vertex| {
            *vertex = to_screen_space(*vertex, game_state);
        });
    }

    /// Actually go and render the geometry
    pub fn render(&self, game_state: &GameState) {
        // Render our geometry
        self.index_buffer
            .iter()
            .enumerate()
            .filter(|(index, _)| self.cull_flags[*index] == false)
            .for_each(|(index, triangle)| {
                let a = self.vertex_buffer[triangle.0].xy();
                let b = self.vertex_buffer[triangle.1].xy();
                let c = self.vertex_buffer[triangle.2].xy();

                let triangle = Triangle {
                    verticies: [a, b, c],
                };

                draw_triangle(triangle, game_state.colors[index]);
            });
    }
}

fn to_screen_space(vec: Vector3<f32>, game_state: &GameState) -> Vector3<f32> {
    let z_inv = vec.z.recip();
    let x = (vec.x * z_inv + 1.0) * (game_state.screen_width as f32 / 2.0);
    let y = (-vec.y * z_inv + 1.0) * (game_state.screen_height as f32 / 2.0);
    let z = vec.z;

    Vector3::new(x, y, z)
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

const CUBE_COLORS: [Color; 8] = [
    Color::new(0, 0, 0xFF),
    Color::new(0, 0xFF, 0),
    Color::new(0, 0xFF, 0xFF),
    Color::new(0xFF, 0, 0),
    Color::new(0xFF, 0, 0xFF),
    Color::new(0xFF, 0xFF, 0),
    Color::new(0xFF, 0xFF, 0xFF),
    Color::new(0xFF, 0, 0xFF),
];
