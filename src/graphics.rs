use crate::{
    gpu::ZBuffer,
    shaders::pixel_shader::PixelShader,
    types::{Triangle, TriangleVertex},
};

use gamercade_rs::prelude as gc;

pub fn draw_triangle<PS: PixelShader<D>, const D: usize>(
    mut triangle: Triangle<D>,
    z_buffer: &mut ZBuffer,
) {
    // Sort verts from top (low) to bottom (high)
    triangle
        .vertices
        .sort_by(|a, b| a.position.y.total_cmp(&b.position.y));

    if triangle.vertices[0].position.y == triangle.vertices[1].position.y {
        // Flat Top
        // We want to go left -> right
        if triangle.vertices[0].position.x > triangle.vertices[1].position.x {
            triangle.vertices.swap(0, 1);
        }
        draw_flat_top_triangle::<PS, D>(triangle, z_buffer);
    } else if triangle.vertices[1].position.y == triangle.vertices[2].position.y {
        // Flat Bottom
        // We want to go left -> right
        if triangle.vertices[1].position.x > triangle.vertices[2].position.x {
            triangle.vertices.swap(1, 2);
        }
        draw_flat_bottom_triangle::<PS, D>(triangle, z_buffer);
    } else {
        // Split the triangle into a flat top and flat bottom triangle
        let alpha = (triangle.vertices[1].position.y - triangle.vertices[0].position.y)
            / (triangle.vertices[2].position.y - triangle.vertices[0].position.y);

        let split = TriangleVertex {
            position: triangle.vertices[0]
                .position
                .lerp(&triangle.vertices[2].position, alpha),
            parameters: triangle.vertices[0]
                .parameters
                .lerp(&triangle.vertices[2].parameters, alpha),
        };

        if split.position.x > triangle.vertices[1].position.x {
            // Split is on the right side
            draw_flat_bottom_triangle::<PS, D>(
                Triangle {
                    vertices: [triangle.vertices[0], triangle.vertices[1], split],
                },
                z_buffer,
            );
            draw_flat_top_triangle::<PS, D>(
                Triangle {
                    vertices: [triangle.vertices[1], split, triangle.vertices[2]],
                },
                z_buffer,
            );
        } else {
            // Split is on the left side
            draw_flat_bottom_triangle::<PS, D>(
                Triangle {
                    vertices: [triangle.vertices[0], split, triangle.vertices[1]],
                },
                z_buffer,
            );
            draw_flat_top_triangle::<PS, D>(
                Triangle {
                    vertices: [split, triangle.vertices[1], triangle.vertices[2]],
                },
                z_buffer,
            );
        }
    }
}

fn draw_flat_top_triangle<PS: PixelShader<D>, const D: usize>(
    triangle: Triangle<D>,
    z_buffer: &mut ZBuffer,
) {
    let verts = triangle.vertices;
    let delta_y = verts[2].position.y - verts[0].position.y;
    let dit0 = (verts[2] - verts[0]) / delta_y;
    let dit1 = (verts[2] - verts[1]) / delta_y;

    let edge_interpolator = verts[1];

    draw_flat_triangle::<PS, D>(triangle, dit0, dit1, edge_interpolator, z_buffer);
}

fn draw_flat_bottom_triangle<PS: PixelShader<D>, const D: usize>(
    triangle: Triangle<D>,
    z_buffer: &mut ZBuffer,
) {
    let verts = triangle.vertices;
    let delta_y = verts[2].position.y - verts[0].position.y;
    let dit0 = (verts[1] - verts[0]) / delta_y;
    let dit1 = (verts[2] - verts[0]) / delta_y;

    let edge_interpolator = verts[0];

    draw_flat_triangle::<PS, D>(triangle, dit0, dit1, edge_interpolator, z_buffer);
}

fn draw_flat_triangle<PS: PixelShader<D>, const D: usize>(
    triangle: Triangle<D>,
    dv0: TriangleVertex<D>,
    dv1: TriangleVertex<D>,
    mut interpolator_edge_1: TriangleVertex<D>,
    z_buffer: &mut ZBuffer,
) {
    let mut interpolator_edge_0 = triangle.vertices[0];

    let y_start = ((triangle.vertices[0].position.y - 0.5).ceil() as i32).max(0);
    let y_end = ((triangle.vertices[2].position.y - 0.5).ceil() as i32)
        .min(z_buffer.screen_height as i32 - 1);

    interpolator_edge_0 += dv0 * (y_start as f32 + 0.5 - triangle.vertices[0].position.y);
    interpolator_edge_1 += dv1 * (y_start as f32 + 0.5 - triangle.vertices[0].position.y);

    (y_start..y_end).for_each(|y| {
        let x_start = ((interpolator_edge_0.position.x - 0.5).ceil() as i32).max(0);
        let x_end = ((interpolator_edge_1.position.x - 0.5).ceil() as i32)
            .min(z_buffer.screen_width as i32 - 1);

        let mut interpolation_line = interpolator_edge_0;
        let dx = interpolator_edge_1.position.x - interpolator_edge_0.position.x;
        let delta_interpolation_line = (interpolator_edge_1 - interpolation_line) / dx;

        interpolation_line +=
            delta_interpolation_line * (x_start as f32 + 0.5 - interpolator_edge_0.position.x);

        (x_start..x_end).for_each(|x| {
            if z_buffer.test_and_set(x as usize, y as usize, interpolation_line.position.z) {
                let w = interpolation_line.position.w.recip();
                let params = interpolation_line.parameters * w;
                let color = PS::run(params).to_graphics_params();
                gc::set_pixel(color, x, y);
            }

            interpolation_line += delta_interpolation_line;
        });

        interpolator_edge_0 += dv0;
        interpolator_edge_1 += dv1;
    });
}
