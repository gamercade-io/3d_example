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
    pub colors: [i32; 16 * 16 * 16],
    pub vertex_data: Box<[Vector3<f32>]>,
    pub index_data: Box<[IndexedTriangle]>,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub offset_z: f32,
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
            .flat_map(|palette| {
                (0..64).map(move |color| graphics_parameters(palette, 0, 0, color, 0, 0))
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    });

    GPU.write(Gpu::default());
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

    // Prepare the data for rendering.
    gpu.prepare_scene(game_state);

    // Call draw etc
    gpu.render(game_state);

    // Clear our buffers for next frame
    gpu.clear()
}
