use std::{f32::consts::PI, mem::MaybeUninit};

use nalgebra::Vector3;

mod console;
use console::*;

mod graphics;

mod gpu;
use gpu::*;

mod types;
use types::IndexedTriangle;

pub struct GameState {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dt: f32,
    pub colors: [i32; 32 * 32 * 16],
    pub vertex_data: Box<[Vector3<f32>]>,
    pub index_data: Box<[IndexedTriangle]>,
    pub vertex_shader_inputs: Box<[Vector3<f32>]>,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub camera_position: Vector3<f32>,
}

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu<Vector3<f32>>> = MaybeUninit::uninit();

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

/// # Safety
/// This function calls external Gamercade Api Functions
pub unsafe fn log(text: &str) {
    // Casting a pointer to an i32 is giving us the memory address.
    let addr = text.as_ptr() as i32;

    console_log(addr, text.len() as i32)
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn init() {
    GAME_STATE.write(GameState {
        screen_width: width(),
        screen_height: height(),
        dt: frame_time(),
        vertex_data: Box::new(cube(SIDE)),
        index_data: Box::new(CUBE_INDICIES),
        vertex_shader_inputs: CUBE_COLORS
            .iter()
            .map(|color| {
                Vector3::new(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                )
            })
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        camera_position: Vector3::new(0.0, 0.0, 2.0),
        colors: (0..256)
            .flat_map(|palette| {
                (0..64).map(move |color| graphics_parameters(palette, 0, 0, color, 0, 0))
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    });

    GPU.write(Gpu::default());
}

/// # Safety
/// This function calls external Gamercade Api Functions
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
        game_state.camera_position.z += ROT_SPEED;
    } else if button_d_held(0) != 0 {
        game_state.camera_position.z -= ROT_SPEED;
    }
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    // Some local working data
    let game_state = GAME_STATE.assume_init_ref();
    let gpu = GPU.assume_init_mut();

    // Clear the screen every frame
    clear_screen(0);

    gpu.render_scene(game_state);

    // Clear our buffers for next frame
    gpu.clear()
}
