use image::{ImageReader, Pixel, RgbaImage};
use ratatui::prelude::*;
use std::io::Cursor;

pub struct TextImage {
    img: RgbaImage,
}

impl TextImage {
    pub fn new(data: Vec<u8>) -> Self {
        let reader = ImageReader::new(Cursor::new(data))
            .with_guessed_format()
            .expect("Cursor IO never fails");
        let image = reader.decode().expect("Image is valid");
        Self {
            img: image.to_rgba8(),
        }
    }
}

impl Widget for &TextImage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = self.img.dimensions();
        let mut rows = vec![];
        for r in (0..height - 1).step_by(2) {
            let mut row = vec![];
            for c in 0..width {
                let p_upper = self.img.get_pixel(c, r);
                let c_upper = p_upper.channels();
                let bg_color = Color::from((c_upper[0], c_upper[1], c_upper[2], c_upper[3]));
                let p_lower = self.img.get_pixel(c, r + 1);
                let c_lower = p_lower.channels();
                let fg_color = Color::from((c_lower[0], c_lower[1], c_lower[2], c_lower[3]));
                // unicode 2584 is "lower half block" graphical character
                // "lower half block" is filled fg, upper part is left bg
                row.push(Span::raw("\u{2584}").bg(bg_color).fg(fg_color));
            }
            rows.push(Line::from(row));
        }
        Text::from(rows).render(area, buf);
    }
}
