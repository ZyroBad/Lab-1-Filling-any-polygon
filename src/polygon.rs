use std::cmp::Ordering;

use raylib::prelude::*;

use crate::framebuffer::Framebuffer;
use crate::line::line_bresenham;

/// Rellena uno o varios contornos usando el algoritmo Scanline.
///
/// Cuando se pasan dos contornos, la regla par-impar permite que el
/// segundo contorno funcione como agujero.
pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    contours: &[&[Vector2]],
    fill_color: Color,
) {
    if contours.is_empty() {
        return;
    }

    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for contour in contours {
        for point in *contour {
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
        }
    }

    framebuffer.set_current_color(fill_color);

    let first_y = min_y.floor() as i32;
    let last_y = max_y.ceil() as i32;

    for y in first_y..=last_y {
        // Se usa el centro vertical del píxel.
        let scan_y = y as f32 + 0.5;

        let mut intersections: Vec<f32> = Vec::new();

        for contour in contours {
            if contour.len() < 3 {
                continue;
            }

            for i in 0..contour.len() {
                let current = contour[i];
                let next = contour[(i + 1) % contour.len()];

                // Las aristas horizontales no producen intersecciones.
                if (current.y - next.y).abs() < f32::EPSILON {
                    continue;
                }

                let min_edge_y = current.y.min(next.y);
                let max_edge_y = current.y.max(next.y);

                /*
                 * Intervalo semiabierto:
                 *
                 * min_y <= scan_y < max_y
                 *
                 * Esto evita contar dos veces los vértices compartidos
                 * entre dos aristas.
                 */
                if scan_y >= min_edge_y && scan_y < max_edge_y {
                    let t = (scan_y - current.y) / (next.y - current.y);
                    let intersection_x =
                        current.x + t * (next.x - current.x);

                    intersections.push(intersection_x);
                }
            }
        }

        intersections.sort_by(|a, b| {
            a.partial_cmp(b).unwrap_or(Ordering::Equal)
        });

        /*
         * Regla par-impar:
         *
         * Se rellena entre la intersección 0 y 1,
         * luego entre la 2 y 3, etc.
         *
         * En el polígono 4, las intersecciones del polígono 5
         * generan automáticamente el agujero.
         */
        for pair in intersections.chunks_exact(2) {
            let left = pair[0];
            let right = pair[1];

            /*
             * Se comprueba el centro horizontal del píxel.
             * Esto reduce los píxeles que podrían salirse del borde.
             */
            let start_x = (left - 0.5).ceil() as i32;
            let end_x = (right - 0.5).floor() as i32;

            for x in start_x..=end_x {
                framebuffer.point(x, y);
            }
        }
    }
}

/// Dibuja el borde cerrado de un polígono.
pub fn draw_polygon_outline(
    framebuffer: &mut Framebuffer,
    vertices: &[Vector2],
    line_color: Color,
) {
    if vertices.len() < 2 {
        return;
    }

    framebuffer.set_current_color(line_color);

    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = vertices[(i + 1) % vertices.len()];

        line_bresenham(framebuffer, start, end);
    }
}

/// Rellena el polígono y después dibuja sus bordes.
///
/// `contours` puede contener:
/// - Un contorno para un polígono normal.
/// - Dos contornos para un polígono con agujero.
pub fn draw_filled_polygon(
    framebuffer: &mut Framebuffer,
    contours: &[&[Vector2]],
    fill_color: Color,
    line_color: Color,
) {
    fill_polygon(framebuffer, contours, fill_color);

    // El borde se dibuja al final para que quede claramente visible.
    for contour in contours {
        draw_polygon_outline(
            framebuffer,
            contour,
            line_color,
        );
    }
}