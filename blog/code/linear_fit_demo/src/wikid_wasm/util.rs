use image::Rgba;

#[derive(Clone, Copy, Debug)]
pub enum Dim {
    Pixel(u32),
    Percent(f64),
}

impl Dim {
    pub fn to_pixel(self, window_width: u32, window_height: u32, horiz: bool) -> u32 {
        match self {
            Dim::Pixel(x) => x,
            Dim::Percent(p) =>
                if horiz {
                    (window_width as f64 * p) as u32
                } else {
                    (window_height as f64 * p) as u32
                },
        }
    }
}

pub fn hex_to_rgba(color: &str) -> Rgba<u8> {
    if !color.starts_with('#') {
        panic!("The color must begin with a hash");
    }
    let r = u8::from_str_radix(&color[1..3], 16).unwrap();
    let g = u8::from_str_radix(&color[3..5], 16).unwrap();
    let b = u8::from_str_radix(&color[5..7], 16).unwrap();
    Rgba([r, g, b, 255])
}

// Find the minimum finite value of an array
pub fn fnanmin(xs: &[f32]) -> f32 {
    let mut out = None;
    for x in xs {
        if !x.is_finite() {continue;}
        out = match out {
            None => Some(*x),
            Some(o) => Some(o.min(*x)),
        }
    }
    out.expect("This array contained no finite values")
}

// Find the maximum finite value of an array
pub fn fnanmax(xs: &[f32]) -> f32 {
    let mut out = None;
    for x in xs {
        if !x.is_finite() {continue;}
        out = match out {
            None => Some(*x),
            Some(o) => Some(o.max(*x)),
        }
    }
    out.expect("This array contained no finite values")
}

pub fn blend_color(a: Rgba<u8>, b: Rgba<u8>, alpha: f32) -> Rgba<u8> {
    Rgba([
        (a.0[0] as f32 * alpha) as u8 + (b.0[0] as f32 * (1. - alpha)) as u8,
        (a.0[1] as f32 * alpha) as u8 + (b.0[1] as f32 * (1. - alpha)) as u8,
        (a.0[2] as f32 * alpha) as u8 + (b.0[2] as f32 * (1. - alpha)) as u8,
        255
    ])
}