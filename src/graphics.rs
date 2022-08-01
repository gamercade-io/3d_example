use nalgebra::Vector3;

use crate::{
    console::set_pixel,
    types::{Color, Triangle, TriangleInner},
    GameState,
};

pub fn draw_triangle(mut triangle: Triangle<Vector3<f32>>, game_state: &GameState) {
    // Sort verts from top (low) to bottom (high)
    triangle
        .verticies
        .sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    if triangle.verticies[0].position.y == triangle.verticies[1].position.y {
        // Flat Top
        // We want to go left -> right
        if triangle.verticies[0].position.x > triangle.verticies[1].position.x {
            triangle.verticies.swap(0, 1);
        }
        draw_flat_top_triangle(triangle, game_state);
    } else if triangle.verticies[1].position.y == triangle.verticies[2].position.y {
        // Flat Bottom
        // We want to go left -> right
        if triangle.verticies[1].position.x > triangle.verticies[2].position.x {
            triangle.verticies.swap(1, 2);
        }
        draw_flat_bottom_triangle(triangle, game_state);
    } else {
        // Split the triangle into a flat top and flat bottom triangle
        let alpha = (triangle.verticies[1].position.y - triangle.verticies[0].position.y)
            / (triangle.verticies[2].position.y - triangle.verticies[0].position.y);

        let split = TriangleInner {
            position: triangle.verticies[0]
                .position
                .lerp(&triangle.verticies[2].position, alpha),
            parameters: triangle.verticies[0]
                .parameters
                .lerp(&triangle.verticies[2].parameters, alpha),
        };

        if split.position.x > triangle.verticies[1].position.x {
            // Split is on the right side
            draw_flat_bottom_triangle(
                Triangle {
                    verticies: [triangle.verticies[0], triangle.verticies[1], split],
                },
                game_state,
            );
            draw_flat_top_triangle(
                Triangle {
                    verticies: [triangle.verticies[1], split, triangle.verticies[2]],
                },
                game_state,
            );
        } else {
            // Split is on the left side
            draw_flat_bottom_triangle(
                Triangle {
                    verticies: [triangle.verticies[0], split, triangle.verticies[1]],
                },
                game_state,
            );
            draw_flat_top_triangle(
                Triangle {
                    verticies: [split, triangle.verticies[1], triangle.verticies[2]],
                },
                game_state,
            );
        }
    }
}

fn draw_flat_top_triangle(triangle: Triangle<Vector3<f32>>, game_state: &GameState) {
    let verts = triangle.verticies;
    let delta_y = verts[2].position.y - verts[0].position.y;
    let dit0 = (verts[2] - verts[0]) / delta_y;
    let dit1 = (verts[2] - verts[1]) / delta_y;

    let edge_interpolator = verts[1];

    draw_flat_triangle(triangle, dit0, dit1, edge_interpolator, game_state);
}

fn draw_flat_bottom_triangle(triangle: Triangle<Vector3<f32>>, game_state: &GameState) {
    let verts = triangle.verticies;
    let delta_y = verts[2].position.y - verts[0].position.y;
    let dit0 = (verts[1] - verts[0]) / delta_y;
    let dit1 = (verts[2] - verts[0]) / delta_y;

    let edge_interpolator = verts[0];

    draw_flat_triangle(triangle, dit0, dit1, edge_interpolator, game_state);
}

fn draw_flat_triangle(
    triangle: Triangle<Vector3<f32>>,
    dv0: TriangleInner<Vector3<f32>>,
    dv1: TriangleInner<Vector3<f32>>,
    mut interpolator_edge_1: TriangleInner<Vector3<f32>>,
    game_state: &GameState,
) {
    let mut interpolator_edge_0 = triangle.verticies[0];

    let y_start = (triangle.verticies[0].position.y - 0.5).ceil() as i32;
    let y_end = (triangle.verticies[2].position.y - 0.5).ceil() as i32;

    interpolator_edge_0 += dv0 * (y_start as f32 + 0.5 - triangle.verticies[0].position.y);
    interpolator_edge_1 += dv1 * (y_start as f32 + 0.5 - triangle.verticies[0].position.y);

    (y_start..y_end).for_each(|y| {
        let x_start = (interpolator_edge_0.position.x - 0.5).ceil() as i32;
        let x_end = (interpolator_edge_1.position.x - 0.5).ceil() as i32;

        let mut interpolation_line = interpolator_edge_0;
        let dx = interpolator_edge_1.position.x - interpolator_edge_0.position.x;
        let delta_interpolation_line = (interpolator_edge_1 - interpolation_line) / dx;

        interpolation_line +=
            delta_interpolation_line * (x_start as f32 + 0.5 - interpolator_edge_0.position.x);

        (x_start..x_end).for_each(|x| unsafe {
            //TODO: Check ZBuffer
            let color_index =
                vertex_color_shader(interpolation_line.parameters).to_554_index();
            set_pixel(game_state.colors[color_index], x, y);

            interpolation_line += delta_interpolation_line;
        });

        interpolator_edge_0 += dv0;
        interpolator_edge_1 += dv1;
    });
}

fn vertex_color_shader(shader_params: Vector3<f32>) -> Color {
    Color {
        r: (shader_params.x * 255.0) as u8,
        g: (shader_params.y * 255.0) as u8,
        b: (shader_params.z * 255.0) as u8,
    }
}
