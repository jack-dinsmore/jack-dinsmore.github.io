use image::Rgba;
use rusttype::{point, Font, Scale};

use super::{blend_color, hex_to_rgba};

const FONT_SCALE: f32 = 1.5;
pub const BLACK: image::Rgba<u8> = Rgba([0, 0, 0, 255]);
pub const WHITE: image::Rgba<u8> = Rgba([255, 255, 255, 255]);

pub enum TextAlign {
    UpperLeft,
    LowerRight,
    Center,
}

pub struct Style {
    pub font: Font<'static>,
    pub font_size: u32,
    pub highlight_color: Rgba<u8>,
    pub point_radius: f32,
    pub line_width: f32, 
}

impl Style {
    /// Load a font using include_bytes
    pub fn default(font_data: &'static [u8]) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Font file is corrupted");
        Self {
            font,
            font_size: 16,
            highlight_color: hex_to_rgba("#888888"),
            point_radius: 4.,
            line_width: 3.,
        }
    }

    pub fn set_color(&mut self, color: &str) {
        self.highlight_color = hex_to_rgba(color);
    }

    pub fn render_text(&self, pixels: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, start_x: u32, start_y: u32, text: &str, color: image::Rgba<u8>, va: TextAlign, ha: TextAlign) {
        let scale = Scale {
            x: self.font_size as f32 * FONT_SCALE,
            y: self.font_size as f32 * FONT_SCALE,
        };
        let scaled_font_size = (self.font_size as f32 * FONT_SCALE) as u32;
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        let start_x = match ha {
            TextAlign::UpperLeft => start_x as i32,
            TextAlign::LowerRight => start_x as i32 - width as i32,
            TextAlign::Center => start_x as i32 - width as i32 / 2,
        };

        let start_y = match va {
            TextAlign::UpperLeft => start_y as i32,
            TextAlign::LowerRight => start_y as i32 - scaled_font_size as i32,
            TextAlign::Center => start_y as i32 - (scaled_font_size as i32) / 2,
        };

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < width as i32 && y >= 0 && y < scaled_font_size as i32 {
                        let x = x + start_x;
                        let y = y + start_y;
                        if x < 0 || y < 0 || x >= pixels.width() as i32 || y >= pixels.height() as i32 {return;}
                        let empty = pixels[(x as u32, y as u32)];
                        pixels[(x as u32, y as u32)] = blend_color(color, empty, v);
                    }
                });
            }
        }
    }

    pub fn render_rotated_text(&self, pixels: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, start_x: u32, start_y: u32, text: &str, color: image::Rgba<u8>, va: TextAlign, ha: TextAlign, rotation: f32) {
        let scale = Scale {
            x: self.font_size as f32 * FONT_SCALE,
            y: self.font_size as f32 * FONT_SCALE,
        };
        let scaled_font_size = (self.font_size as f32 * FONT_SCALE) as u32;
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();

        // Find the most visually pleasing width to display
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as usize;

        let offset_local_x = match ha {
            TextAlign::UpperLeft => 0.,
            TextAlign::LowerRight => -(width as f32),
            TextAlign::Center => -(width as f32) / 2.,
        };

        let offset_local_y = match va {
            TextAlign::UpperLeft => 0.,
            TextAlign::LowerRight => -(scaled_font_size as f32),
            TextAlign::Center => -(scaled_font_size as f32) / 2.,
        };

        let rot_rad = rotation * std::f32::consts::PI / 180.;
        let start_x = (start_x as f32 + offset_local_x * rot_rad.cos() + offset_local_y * rot_rad.sin()) as i32;
        let start_y = (start_y as f32 + offset_local_y * rot_rad.cos() - offset_local_x * rot_rad.sin()) as i32;

        for g in glyphs {
            if let Some(bb) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < width as i32 && y >= 0 && y < scaled_font_size as i32 {
                        // Rotate
                        let (x, y) = if rotation == 0. {
                            (x, y)
                        } else if rotation == 90. {
                            (y, -x)
                        } else if rotation == 180. {
                            (-x, -y)
                        } else if rotation == 270. {
                            (-y, x)
                        } else {
                            unimplemented!()
                        };

                        let x = x + start_x;
                        let y = y + start_y;
                        if x < 0 || y < 0 || x >= pixels.width() as i32 || y >= pixels.height() as i32 {return;}
                        let empty = pixels[(x as u32, y as u32)];
                        pixels[(x as u32, y as u32)] = blend_color(color, empty, v);
                    }
                });
            }
        }
    }
}