use std::{f32::consts::PI, mem::MaybeUninit};

use nalgebra::{Rotation3, Vector3};

mod console;
use console::*;

use crate::graphics::{draw_triangle, Triangle};

mod graphics;

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dt: f32,
    pub colors: [i32; 64],
    pub vertex_data: Box<[Vector3<f32>]>,
    pub index_data: Box<[IndexedTriangle]>,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub offset_z: f32,
}

pub struct Gpu {
    vertex_buffer: Vec<Vector3<f32>>,
    index_buffer: Vec<IndexedTriangle>,
    cull_flags: Vec<bool>,
}

impl Gpu {
    fn clear(&mut self) {
        self.vertex_buffer.clear();
        self.index_buffer.clear();
        self.cull_flags.clear();
    }
}

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu> = MaybeUninit::uninit();

const ROT_SPEED: f32 = PI * 0.01;

const SIDE: f32 = 1.0;
fn cube(size: f32) -> [Vector3<f32>; 8] {
    let side = size * 0.5;
    [
        Vector3::new(-side, -side, -side),
        Vector3::new(side, -side, -side),
        Vector3::new(-side, side, -side),
        Vector3::new(side, side, -side),
        Vector3::new(-side, -side, side),
        Vector3::new(side, -side, side),
        Vector3::new(-side, side, side),
        Vector3::new(side, side, side),
    ]
}

pub struct TriangleEdge(usize, usize);
const CUBE_EDGES: [TriangleEdge; 12] = [
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

#[derive(Clone, Copy)]
pub struct IndexedTriangle(usize, usize, usize);
const CUBE_INDICIES: [IndexedTriangle; 12] = [
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

fn to_screen_space(vec: Vector3<f32>, game_state: &GameState) -> Vector3<f32> {
    let z_inv = vec.z.recip();
    let x = (vec.x * z_inv + 1.0) * (game_state.screen_width as f32 / 2.0);
    let y = (-vec.y * z_inv + 1.0) * (game_state.screen_height as f32 / 2.0);
    let z = vec.z;

    Vector3::new(x, y, z)
}

/// # Safety
/// This function calls external Gamercade Api Functions
pub unsafe fn log(text: &str) {
    // Casting a pointer to an i32 is giving us the memory address.
    let addr = text.as_ptr() as i32;

    console_log(addr, text.len() as i32)
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    GAME_STATE.write(GameState {
        screen_width: width(),
        screen_height: height(),
        dt: frame_time(),
        vertex_data: Box::new(cube(SIDE)),
        index_data: Box::new(CUBE_INDICIES),
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        offset_z: 2.0,
        colors: (0..64)
            .map(|index| graphics_parameters(8, 0, 0, index, 0, 0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    });

    GPU.write(Gpu {
        vertex_buffer: Vec::new(),
        index_buffer: Vec::new(),
        cull_flags: Vec::new(),
    });
}

#[no_mangle]
pub unsafe extern "C" fn update() {
    let game_state = GAME_STATE.assume_init_mut();

    if button_a_held(0) != 0 {
        game_state.yaw += ROT_SPEED;
    } else if button_b_held(0) != 0 {
        game_state.yaw -= ROT_SPEED;
    }

    if button_up_held(0) != 0 {
        game_state.roll += ROT_SPEED;
    } else if button_down_held(0) != 0 {
        game_state.roll -= ROT_SPEED
    }

    if button_right_held(0) != 0 {
        game_state.pitch -= ROT_SPEED;
    } else if button_left_held(0) != 0 {
        game_state.pitch += ROT_SPEED
    }

    if button_c_held(0) != 0 {
        game_state.offset_z += ROT_SPEED;
    } else if button_d_held(0) != 0 {
        game_state.offset_z -= ROT_SPEED;
    }
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    // Some local working data
    let game_state = GAME_STATE.assume_init_ref();
    let gpu = GPU.assume_init_mut();

    // Clear the screen every frame
    clear_screen(0);

    let rot = Rotation3::from_euler_angles(game_state.roll, game_state.pitch, game_state.yaw);

    // Transform our geometry and push it into the gpu
    gpu.vertex_buffer
        .extend(game_state.vertex_data.iter().map(|vertex| {
            let mut vertex = rot * vertex;
            vertex.z += game_state.offset_z;
            vertex
        }));

    // Build the index list, and check for backfacing tris
    gpu.index_buffer
        .extend(game_state.index_data.iter().map(|indicies| {
            let verts = [
                gpu.vertex_buffer[indicies.0],
                gpu.vertex_buffer[indicies.1],
                gpu.vertex_buffer[indicies.2],
            ];

            let cross_result = (verts[1] - verts[0]).cross(&(verts[2] - verts[0]));
            let dot_result = cross_result.dot(&verts[0]);
            let cull_flag = dot_result > 0.0;
            gpu.cull_flags.push(cull_flag);

            IndexedTriangle(indicies.0, indicies.1, indicies.2)
        }));

    // Convert the verticies to screen space
    gpu.vertex_buffer.iter_mut().for_each(|vertex| {
        *vertex = to_screen_space(*vertex, game_state);
    });

    // Render our geometry
    gpu.index_buffer
        .iter()
        .enumerate()
        .filter(|(index, _)| gpu.cull_flags[*index] == false)
        .for_each(|(index, triangle)| {
            let a = gpu.vertex_buffer[triangle.0].xy();
            let b = gpu.vertex_buffer[triangle.1].xy();
            let c = gpu.vertex_buffer[triangle.2].xy();

            let triangle = Triangle {
                verticies: [a, b, c],
            };

            draw_triangle(triangle, game_state.colors[index]);
        });

    // Clear our buffers for next frame
    gpu.clear()
}
