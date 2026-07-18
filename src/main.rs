mod framebuffer;
mod line;
mod polygon;

use framebuffer::Framebuffer;
use polygon::draw_filled_polygon;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 450;

    let background = Color::new(245, 245, 240, 255);

    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(background);
    framebuffer.clear();

    // --------------------------------------------------
    // Polígono 1
    // --------------------------------------------------

    let polygon_1 = [
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];

    draw_filled_polygon(
        &mut framebuffer,
        &[&polygon_1],
        Color::new(225, 65, 70, 255),
        Color::new(120, 25, 30, 255),
    );

    // --------------------------------------------------
    // Polígono 2
    // --------------------------------------------------

    let polygon_2 = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];

    draw_filled_polygon(
        &mut framebuffer,
        &[&polygon_2],
        Color::new(70, 145, 215, 255),
        Color::new(25, 75, 125, 255),
    );

    // --------------------------------------------------
    // Polígono 3
    // --------------------------------------------------

    let polygon_3 = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];

    draw_filled_polygon(
        &mut framebuffer,
        &[&polygon_3],
        Color::new(240, 190, 55, 255),
        Color::new(135, 90, 10, 255),
    );

    // --------------------------------------------------
    // Polígono 4
    // Coordenadas originales
    // --------------------------------------------------

    let polygon_4 = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];

    draw_filled_polygon(
        &mut framebuffer,
        &[&polygon_4],
        Color::new(85, 180, 110, 255),
        Color::new(20, 100, 50, 255),
    );

    // --------------------------------------------------
    // Polígono 5
    // Agujero dentro del polígono 4
    // --------------------------------------------------

    let polygon_5 = [
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];

    /*
     * Se rellena con el color de fondo para crear el agujero.
     * Después se dibuja su borde.
     */
   draw_filled_polygon(
    &mut framebuffer,
    &[&polygon_4, &polygon_5],
    Color::new(85, 180, 110, 255),
    Color::new(20, 100, 50, 255),
);

    framebuffer.render_to_file("out.bmp");

    println!("Imagen generada correctamente: out.bmp");
}