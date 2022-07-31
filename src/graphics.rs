use nalgebra::Vector2;

use crate::{console::set_pixel, log};

pub struct Triangle {
    pub verticies: [Vector2<f32>; 3],
}

pub fn draw_triangle(mut triangle: Triangle, graphics_parameters: i32) {
    // Sort verts from top (low) to bottom (high)
    triangle
        .verticies
        .sort_unstable_by(|a, b| a.y.total_cmp(&b.y));

    if triangle.verticies[0].y == triangle.verticies[1].y {
        // Flat Top
        // We want to go left -> right
        if triangle.verticies[0].x > triangle.verticies[1].x {
            triangle.verticies.swap(0, 1);
        }
        draw_flat_top_triangle(triangle, graphics_parameters);
    } else if triangle.verticies[1].y == triangle.verticies[2].y {
        // Flat Bottom
        // We want to go left -> right
        if triangle.verticies[1].x > triangle.verticies[2].x {
            triangle.verticies.swap(1, 2);
        }
        draw_flat_bottom_triangle(triangle, graphics_parameters);
    } else {
        // Split the triangle into a flat top and flat bottom triangle
        let split = (triangle.verticies[1].y - triangle.verticies[0].y)
            / (triangle.verticies[2].y - triangle.verticies[0].y);
        let split =
            triangle.verticies[0] + ((triangle.verticies[2] - triangle.verticies[0]) * split);

        if split.x > triangle.verticies[1].x {
            // Split is on the right side
            draw_flat_bottom_triangle(
                Triangle {
                    verticies: [triangle.verticies[0], triangle.verticies[1], split],
                },
                graphics_parameters,
            );
            draw_flat_top_triangle(
                Triangle {
                    verticies: [triangle.verticies[1], split, triangle.verticies[2]],
                },
                graphics_parameters,
            );
        } else {
            // Split is on the left side
            draw_flat_bottom_triangle(
                Triangle {
                    verticies: [triangle.verticies[0], split, triangle.verticies[1]],
                },
                graphics_parameters,
            );
            draw_flat_top_triangle(
                Triangle {
                    verticies: [split, triangle.verticies[1], triangle.verticies[2]],
                },
                graphics_parameters,
            );
        }
    }
}

fn draw_flat_top_triangle(triangle: Triangle, graphics_parameters: i32) {
    let m0 = (triangle.verticies[2].x - triangle.verticies[0].x)
        / (triangle.verticies[2].y - triangle.verticies[0].y);
    let m1 = (triangle.verticies[2].x - triangle.verticies[1].x)
        / (triangle.verticies[2].y - triangle.verticies[1].y);

    let y_start = (triangle.verticies[0].y - 0.5).ceil() as i32;
    let y_end = (triangle.verticies[2].y - 0.5).ceil() as i32;

    (y_start..y_end).for_each(|y| {
        let px0 = m0 * (y as f32 + 0.5 - triangle.verticies[0].y) + triangle.verticies[0].x;
        let px1 = m1 * (y as f32 + 0.5 - triangle.verticies[1].y) + triangle.verticies[1].x;

        let x_start = (px0 - 0.5).ceil() as i32;
        let x_end = (px1 - 0.5).ceil() as i32;

        (x_start..x_end).for_each(|x| unsafe {
            set_pixel(graphics_parameters, x, y);
        })
    });
}

fn draw_flat_bottom_triangle(triangle: Triangle, graphics_parameters: i32) {
    let m0 = (triangle.verticies[1].x - triangle.verticies[0].x)
        / (triangle.verticies[1].y - triangle.verticies[0].y);
    let m1 = (triangle.verticies[2].x - triangle.verticies[0].x)
        / (triangle.verticies[2].y - triangle.verticies[0].y);

    let y_start = (triangle.verticies[0].y - 0.5).ceil() as i32;
    let y_end = (triangle.verticies[2].y - 0.5).ceil() as i32;

    (y_start..y_end).for_each(|y| {
        let px0 = m0 * (y as f32 + 0.5 - triangle.verticies[0].y) + triangle.verticies[0].x;
        let px1 = m1 * (y as f32 + 0.5 - triangle.verticies[0].y) + triangle.verticies[0].x;

        let x_start = (px0 - 0.5).ceil() as i32;
        let x_end = (px1 - 0.5).ceil() as i32;

        (x_start..x_end).for_each(|x| unsafe {
            set_pixel(graphics_parameters, x, y);
        })
    });
}
