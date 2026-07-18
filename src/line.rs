use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

pub fn line_bresenham(
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;

    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();

    let step_x = if x0 < x1 { 1 } else { -1 };
    let step_y = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        framebuffer.point(x0, y0);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let double_error = 2 * error;

        if double_error >= dy {
            error += dy;
            x0 += step_x;
        }

        if double_error <= dx {
            error += dx;
            y0 += step_y;
        }
    }
}