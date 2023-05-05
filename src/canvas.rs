use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Context;

use crate::color::{Color, Colors};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub(crate) struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut Color> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.pixels[coord.y * self.width + coord.x])
    }

    pub fn cell(&self, coord: GridCoord) -> Option<&Color> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.pixels[coord.y * self.width + coord.x])
    }

    /// write color changes the pixel at the grid coordinate to the new color.
    /// panics if trying to write out of the canvas bounds.
    pub fn write_color(&mut self, coord: GridCoord, color: Color) {
        let cell = self.cell_mut(coord).unwrap();
        *cell = color
    }

    /// writes the canvas as a png file to the given file path.
    pub fn write_file(self, file: File) -> anyhow::Result<()> {
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);

        let mut writer = encoder
            .write_header()
            .context("failed to write image header")?;

        writer
            .write_image_data(&self.to_image_data())
            .context("failed to write image data")
    }

    pub fn to_image_data(self) -> Vec<u8> {
        use Colors::*;
        self.pixels
            .iter()
            .map(|color| [color[R] as u8, color[G] as u8, color[B] as u8])
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod test_canvas {

    use super::*;

    #[test]
    fn create_canvas() {
        let mut canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for pixel in canvas.pixels.iter() {
            assert_eq!(Color::new(0.0, 0.0, 0.0), *pixel)
        }

        canvas.write_color((0, 0).into(), Color::new(1.0, 1.0, 1.0));

        let color = canvas.cell((0, 0).into());

        assert_eq!(Color::new(1.0, 1.0, 1.0), *color.unwrap())
    }
}
