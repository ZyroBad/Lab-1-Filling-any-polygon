use raylib::ffi;
use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let background_color = Color::BLACK;

        let image = unsafe {
            Image::from_raw(
                ffi::GenImageColor(
                    width,
                    height,
                    background_color,
                )
            )
        };

        Self {
            image,
            width,
            height,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        let new_image = unsafe {
            Image::from_raw(
                ffi::GenImageColor(
                    self.width,
                    self.height,
                    self.background_color,
                )
            )
        };

        self.image = new_image;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        if x < 0 || x >= self.width {
            return;
        }

        if y < 0 || y >= self.height {
            return;
        }

        self.image.draw_pixel(
            x,
            y,
            self.current_color,
        );
    }

    pub fn render_to_file(&self, file_name: &str) {
        self.image.export_image(file_name);

        println!(
            "Proceso de exportación terminado: {}",
            file_name
        );
    }

}