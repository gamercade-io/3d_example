use std::{f32::consts::PI, mem::MaybeUninit};

use nalgebra::{Rotation3, Vector3};

mod console;
use console::*;

#[derive(Default)]
pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dt: f32,
    pub line_colors: [i32; 12],
    pub vertex_buffer: Box<[Vector3<f32>]>,
    pub index_buffer: Box<[IndexedTriangle]>,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub offset_z: f32,
}

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();

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

pub struct IndexedTriangle(usize, usize, usize);
// const CUBE_INDICIES: [IndexedTriangle; 12] = [
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
//     IndexedTriangle( , ,),
// ];

fn to_screen_space(vec: Vector3<f32>, game_state: &GameState) -> Vector3<f32> {
    let z_inv = vec.z.recip();
    let x = (vec.x * z_inv + 1.0) * (game_state.screen_width as f32 / 2.0);
    let y = (-vec.y * z_inv + 1.0) * (game_state.screen_height as f32 / 2.0);
    let z = vec.z;

    Vector3::new(x, y, z)
}

/// # Safety
/// This function calls external Gamercade Api Functions
unsafe fn log(text: &str) {
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
        vertex_buffer: Box::new(cube(SIDE)),
        index_buffer: Box::new([]),
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        offset_z: 2.0,
        line_colors: CUBE_EDGES
            .iter()
            .enumerate()
            .map(|(line, _)| graphics_parameters(8, 0, 0, line as i32 + 1, 0, 0))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
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
    static mut GEOMETRY_BUFFER: Vec<Vector3<f32>> = Vec::new();
    let game_state = GAME_STATE.assume_init_ref();

    // Clear the screen every frame
    clear_screen(0);

    let rot = Rotation3::from_euler_angles(game_state.roll, game_state.pitch, game_state.yaw);

    // Transform our geometry into screen space
    GEOMETRY_BUFFER.extend(game_state.vertex_buffer.iter().map(|vertex| {
        let mut vertex = rot * vertex;
        vertex.z += game_state.offset_z;
        to_screen_space(vertex, game_state)
    }));

    // Render our geometry
    CUBE_EDGES.iter().enumerate().for_each(|(i, edge)| {
        let start = GEOMETRY_BUFFER[edge.0];
        let end = GEOMETRY_BUFFER[edge.1];
        line(
            game_state.line_colors[i],
            start.x as i32,
            start.y as i32,
            end.x as i32,
            end.y as i32,
        );
    });

    // Clear our buffer fo rnext frame
    GEOMETRY_BUFFER.clear();
}
