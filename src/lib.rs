use std::{f32::consts::PI, mem::MaybeUninit};

use nalgebra::{Rotation3, Transform3, Translation3, Vector3};

use gamercade_rs::prelude as gc;
use gamercade_rs::raw;

mod gpu;
mod graphics;
mod image;
mod pipeline;
mod shaders;
mod shapes;
mod types;

use shaders::bind_model_matrix;
use shaders::Textured;
use shaders::{vertex_shader, ColorBlend, DefaultGeometryShader, DefaultVertexShader};
use shapes::plane;
use shapes::PLANE_INDICES;
use shapes::PLANE_UVS;
use shapes::{cube, CUBE_COLORS, CUBE_INCIDES, CUBE_UVS, SIDE};

use gpu::Gpu;
use pipeline::Pipeline;
use types::{IndexedTriangle, RawPoint};

pub struct GameState {
    pub screen_width: usize,
    pub screen_height: usize,
    pub dt: f32,
    pub vertex_data: Box<[RawPoint<2>]>,
    pub index_data: Box<[IndexedTriangle]>,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub camera_position: Vector3<f32>,
}

static mut GAME_STATE: MaybeUninit<GameState> = MaybeUninit::uninit();
static mut PIPELINE: MaybeUninit<Pipeline<2, 2, 2>> = MaybeUninit::uninit();
static mut GPU: MaybeUninit<Gpu> = MaybeUninit::uninit();

const ROT_SPEED: f32 = PI * 0.01;

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn init() {
    let vertex_data_uvs = cube(SIDE)
        .into_iter()
        .zip(CUBE_UVS.into_iter())
        .map(|(position, uvs)| RawPoint {
            position,
            parameters: uvs,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let vertex_data_colored = cube(SIDE)
        .into_iter()
        .zip(CUBE_COLORS.into_iter())
        .map(|(position, color)| RawPoint {
            position,
            parameters: color,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let vertex_data_plane = plane(SIDE)
        .into_iter()
        .zip(PLANE_UVS.into_iter())
        .map(|(position, color)| RawPoint {
            position,
            parameters: color,
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let screen_width = gc::width();
    let screen_height = gc::height();

    PIPELINE.write(Pipeline::new(screen_width, screen_height));
    GPU.write(Gpu::new(screen_width, screen_height));

    vertex_shader::init_projection(screen_width, screen_height);
    bind_model_matrix(Transform3::identity());

    GAME_STATE.write(GameState {
        screen_width,
        screen_height,
        dt: gc::frame_time(),
        vertex_data: vertex_data_plane,
        index_data: Box::new(PLANE_INDICES),
        roll: 0.0,
        pitch: 0.0,
        yaw: 0.0,
        camera_position: Vector3::new(0.0, 0.0, 2.0),
    });
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn update() {
    let game_state = GAME_STATE.assume_init_mut();

    if Some(true) == gc::button_a_held(0) {
        game_state.yaw += ROT_SPEED;
    } else if Some(true) == gc::button_b_held(0) {
        game_state.yaw -= ROT_SPEED;
    }

    if Some(true) == gc::button_up_held(0) {
        game_state.roll += ROT_SPEED;
    } else if Some(true) == gc::button_down_held(0) {
        game_state.roll -= ROT_SPEED
    }

    if Some(true) == gc::button_right_held(0) {
        game_state.pitch -= ROT_SPEED;
    } else if Some(true) == gc::button_left_held(0) {
        game_state.pitch += ROT_SPEED
    }

    if Some(true) == gc::button_c_held(0) {
        game_state.camera_position.z += ROT_SPEED;
    } else if Some(true) == gc::button_d_held(0) {
        game_state.camera_position.z -= ROT_SPEED;
    }

    let view = Transform3::identity()
        * Translation3::from(-game_state.camera_position)
        * Rotation3::from_euler_angles(game_state.roll, game_state.pitch, game_state.yaw);

    vertex_shader::bind_view_matrix(view);
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    // Some local working data
    let game_state = GAME_STATE.assume_init_ref();
    let pipeline = PIPELINE.assume_init_mut();
    let gpu = GPU.assume_init_mut();

    // Clear the screen every frame
    raw::clear_screen(0);
    gpu.clear_z_buffer();

    pipeline.render_scene::<DefaultVertexShader, DefaultGeometryShader, Textured>(
        &game_state.vertex_data,
        &game_state.index_data,
        &mut gpu.z_buffer,
    );
}
