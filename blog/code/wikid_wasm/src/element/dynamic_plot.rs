use image::{ImageBuffer, Rgba};

use crate::{blend_color, fnanmax, fnanmin, Dim};
use crate::style::{Style, TextAlign, BLACK, WHITE};

const DASH_SIZE: f32 = 3.;

use super::Element;

pub struct DynamicPlot {
    pixels: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    top: u32,
    left: u32,
    width: u32,
    height: u32,

    x_lim: (f32, f32),
    y_lim: (f32, f32),
    border_x: u32,
    border_y: u32,
}

pub enum LineStyle {
    Solid,
    Dashed,
    Dotted
}

pub enum PlotCommand<'a> {
    Scatter{xs: &'a[f32], ys: &'a[f32]},
    ErrorBar{xs: &'a[f32], ys: &'a[f32], y_errs: &'a[f32]},
    FillBetween{xs: &'a[f32], y1s: &'a[f32], y2s: &'a[f32]},
    Line{xs: &'a[f32], ys: &'a[f32], ls: LineStyle},
    SetXLim{low: f32, high: f32},
    SetYLim{low: f32, high: f32},
    SetXLabel{label: String},
    SetYLabel{label: String},
    Legend{labels: Vec<Option<String>>},
    Text{ x: f32, y: f32, text: String, va: TextAlign, ha: TextAlign },
}

impl DynamicPlot {
    pub fn new(rect: (Dim, Dim, Dim, Dim), window_width: u32, window_height: u32) -> Self {
        let top = rect.0.to_pixel(window_width, window_height, true);
        let left = rect.1.to_pixel(window_width, window_height, false);
        let width = rect.2.to_pixel(window_width, window_height, true);
        let height = rect.3.to_pixel(window_width, window_height, false);
        Self {
            pixels: ImageBuffer::new(width, height),
            top,
            left,
            width,
            height,
            x_lim: (f32::NAN, f32::NAN),
            y_lim: (f32::NAN, f32::NAN),
            border_x: 0,
            border_y: 0,
        }
    }

    pub fn plot(&mut self, commands: Vec<PlotCommand>, style: &Style) {
        self.clear();
        self.set_axis_limits(&commands);
        self.compute_layout(style);
        self.draw_axis();
        self.draw_ticks(style);
        for command in commands {
            match command {
                PlotCommand::Scatter { xs, ys } => {
                    for i in 0..xs.len() {
                        let point = match self.data_to_axis((xs[i], ys[i])) {
                            Some(p) => p,
                            None => continue
                        };
                        self.draw_disk(point, style.point_radius, style.highlight_color);
                        self.draw_circle(point, style.point_radius, BLACK);
                    }
                },
                PlotCommand::Line { xs, ys, ls } => {
                    let mut remainder = 0.;
                    let mut plot_dash = true;
                    for i in 0..(xs.len()-1) {
                        let point_1 = match self.data_to_axis((xs[i], ys[i])) {
                            Some(p) => p,
                            None => continue
                        };
                        let point_2 = match self.data_to_axis((xs[i+1], ys[i+1])) {
                            Some(p) => p,
                            None => continue
                        };
                        match ls {
                            LineStyle::Solid => {
                                self.draw_line(point_1, point_2, style.line_width, BLACK);
                            },
                            LineStyle::Dashed => {
                                let v = (point_2.0 - point_1.0, point_2.1 - point_1.1);
                                let length = (v.0*v.0 + v.1*v.1).sqrt() * DASH_SIZE * style.line_width * 10.;
                                let v = (v.0 / length, v.1 / length);
                                let mut alpha = remainder;
                                while alpha < length {
                                    if plot_dash {
                                        let end = (alpha.floor() + 1.).min(length);
                                        self.draw_line(
                                            (point_1.0 + v.0 * alpha, point_1.1 + v.1 * alpha),
                                            (point_1.0 + v.0 * end, point_1.1 + v.1 * end)
                                            , style.line_width, BLACK
                                        );
                                    }
                                    alpha = alpha.floor() + 1.;
                                    plot_dash = !plot_dash;
                                }
                                remainder = length - length.floor();
                            }
                            LineStyle::Dotted => {
                                unimplemented!();
                            }
                        }
                    }
                },
                PlotCommand::SetXLabel { label } => {
                    style.render_text(&mut self.pixels, self.border_x + (self.width - self.border_x)/2, self.height - self.border_y / 2, &label, BLACK, TextAlign::Center, TextAlign::Center);
                },
                PlotCommand::SetYLabel { label } => {
                    style.render_rotated_text(&mut self.pixels, self.border_x/2, (self.height - self.border_y) / 2, &label, BLACK, TextAlign::Center, TextAlign::Center, 90.);
                },
                PlotCommand::SetXLim { .. } => (),
                PlotCommand::SetYLim { .. } => (),
                PlotCommand::ErrorBar { xs, ys, y_errs } => {
                    for i in 0..xs.len() {
                        let center_axis = match self.data_to_axis((xs[i], ys[i])) {
                            Some(p) => p,
                            None => continue
                        };
                        let down = match self.data_to_axis((xs[i], ys[i]-y_errs[i])) {
                            Some(p) => self.axis_to_pixel(p),
                            None => self.axis_to_pixel((center_axis.0, 0.)),
                        };
                        let up = match self.data_to_axis((xs[i], ys[i]+y_errs[i])) {
                            Some(p) => self.axis_to_pixel(p),
                            None => self.axis_to_pixel((center_axis.0, 1.)),
                        };
                        self.draw_v_line(down.0.round() as u32, (down.1.round() as u32, up.1.round() as u32), 1, BLACK);
                    }
                },
                PlotCommand::FillBetween { xs, y1s, y2s } => {
                    let alpha = 0.3;
                    for i in 0..xs.len()-1 {
                        let left = (
                            self.data_to_axis((xs[i], y1s[i])),
                            self.data_to_axis((xs[i], y2s[i])),
                        );
                        let right = (
                            self.data_to_axis((xs[i+1], y1s[i+1])),
                            self.data_to_axis((xs[i+1], y2s[i+1])),
                        );
                        if left.0.is_none() || left.1.is_none() || right.0.is_none() || right.1.is_none() {
                            continue;
                        }
                        let left = (
                            self.axis_to_pixel(left.0.unwrap()),
                            self.axis_to_pixel(left.1.unwrap())
                        );
                        let right = (
                            self.axis_to_pixel(right.0.unwrap()),
                            self.axis_to_pixel(right.1.unwrap())
                        );
                        let ul = left.0.1.max(left.1.1);
                        let ll = left.0.1.min(left.1.1);
                        let ur = right.0.1.max(right.1.1);
                        let lr = right.0.1.min(right.1.1);
                        let left = left.0.0.round() as u32;
                        let right = right.0.0.round() as u32;
                        for i in left..right {
                            let frac = (i - left) as f32 / (right - left) as f32;
                            let j_top = ul + frac * (ur - ul);
                            let j_bot = ll + frac * (lr - ll);
                            for j in (j_bot.floor() as u32-1)..=(j_top.ceil() as u32+1) {
                                let empty = self.pixels[(i, j)];
                                if j >= j_bot.ceil() as u32 && j <= j_top.floor() as u32 {
                                    self.pixels[(i, j)] = blend_color(style.highlight_color, empty, alpha);
                                } else {
                                    let mult = 1. + (j as f32 - j_bot).min(j_top - j as f32);
                                    if mult < 0. {continue;}
                                    self.pixels[(i, j)] = blend_color(style.highlight_color, empty, alpha * mult);
                                }
                            }
                        }
                    }
                },
                PlotCommand::Legend { labels } => {
                    unimplemented!();
                },
                PlotCommand::Text { x, y, text, va, ha } => {
                    let (x, y) = self.axis_to_pixel((x, y));
                    style.render_text(&mut self.pixels, x.round() as u32, y.round() as u32, &text, BLACK, va, ha);
                },
            }
        }
    }

    fn clear(&mut self) {
        for (_, _, pixel) in self.pixels.enumerate_pixels_mut() {
            *pixel = WHITE;
        }
    }

    fn set_axis_limits(&mut self, commands: &[PlotCommand]) {
        let mut x_lim = None;
        let mut y_lim = None;
        let mut x_lim_set = false;
        let mut y_lim_set = false;
        for command in commands {
            let (local_x_lim, local_y_lim) = match command {
                PlotCommand::Scatter{ xs, ys, .. } => {
                    (
                        (fnanmin(xs), fnanmax(xs)),
                        (fnanmin(ys), fnanmax(ys))
                    )
                }
                PlotCommand::Line{ xs, ys, .. } => {
                    (
                        (fnanmin(xs), fnanmax(xs)),
                        (fnanmin(ys), fnanmax(ys))
                    )
                }
                PlotCommand::SetXLim{low, high} => {
                    x_lim = Some((*low, *high));
                    x_lim_set = true;
                    continue
                }
                PlotCommand::SetYLim{low, high} => {
                    y_lim = Some((*low, *high));
                    y_lim_set = true;
                    continue
                }
                _ => continue
            };
            if !x_lim_set {
                x_lim = match x_lim {
                    None => Some(local_x_lim),
                    Some(x) => Some((x.0.min(local_x_lim.0), x.1.max(local_x_lim.1)))
                };
            }
            if !y_lim_set {
                y_lim = match y_lim {
                    None => Some(local_y_lim),
                    Some(y) => Some((y.0.min(local_y_lim.0), y.1.max(local_y_lim.1)))
                };
            }
        }
        if !x_lim_set {
            // Widen a bit
            if let Some(a) = x_lim {
                let buffer = (a.0 - a.1).abs() * 0.03;
                x_lim = Some((a.0 - buffer, a.1 + buffer));
            }
        }
        if !y_lim_set {
            // Widen a bit
            if let Some(a) = y_lim {
                let buffer = (a.0 - a.1).abs() * 0.03;
                y_lim = Some((a.0 - buffer, a.1 + buffer));
            }
        }
        self.x_lim = match x_lim {
            Some(a) => a,
            None => (-1., 1.)
        };
        self.y_lim = match y_lim {
            Some(a) => a,
            None => (-1., 1.)
        };
    }

    fn compute_layout(&mut self, style: &Style) {
        self.border_x = 2 * style.font_size as u32;
        self.border_y = 2 * style.font_size as u32;
    }

    fn draw_axis(&mut self) {
        self.draw_v_line(self.border_x, (0, self.height - self.border_y), 1, BLACK);
        self.draw_h_line(self.height - self.border_y, (self.border_x, self.width), 1, BLACK);
    }

    fn draw_ticks(&mut self, style: &Style) {
        // Write email to Susan
        // Do this (ticks)
        // Put into the main website and wikid
        // Write explanatory thing about sigma deviations

        fn get_automatic_ticks(lim: (f32, f32)) -> (Vec<f32>, Vec<f32>) {
            let width = lim.1 - lim.0;
            let main_major_division = 10f64.powi((width.log10()).round() as i32);
            let mut divisor_index = 0;
            let divisors = [1., 2., 5., 10., 20., 50.];
            let major_division = loop {
                let major_division = main_major_division / divisors[divisor_index];
                let count = (width as f64 / major_division) as usize;
                if count > 3 {
                    break major_division;
                }
                if divisor_index < divisors.len() - 1 {
                    divisor_index += 1;
                } else {
                    break major_division;
                }
            };
            let minor_division = major_division / 10.;
            let majors = ((lim.0 as f64 / major_division).ceil() as i32..=(lim.1 as f64 / major_division).floor() as i32).map(|m| (m as f64 * major_division) as f32).collect::<Vec<_>>();
            let minors = ((lim.0 as f64 / minor_division).ceil() as i32..=(lim.1 as f64 / minor_division).floor() as i32).map(|m| (m as f64 * minor_division) as f32).collect::<Vec<_>>();
            (majors, minors)
        }

        let (avg_x, avg_y) = (
            (self.x_lim.1 + self.x_lim.0) / 2.,
            (self.y_lim.1 + self.y_lim.0) / 2.,
        );
        let (x_majors, x_minors) = get_automatic_ticks(self.x_lim);
        let (y_majors, y_minors) = get_automatic_ticks(self.y_lim);
        for major in x_majors {
            let x = self.axis_to_pixel(self.data_to_axis((major, avg_y)).unwrap()).0.round() as u32;
            self.draw_v_line(x, (self.height - self.border_y, self.height - self.border_y - 5), 1, BLACK);
            if (x as i32 - (self.width/2 + self.border_x) as i32).abs() < 20 {
                continue;
            }
            style.render_text(&mut self.pixels, x, self.height - self.border_y, &format!("{}", major), BLACK, TextAlign::UpperLeft, TextAlign::Center);
        }
        for major in y_majors {
            let y = self.axis_to_pixel(self.data_to_axis((avg_x, major)).unwrap()).1.round() as u32;
            self.draw_h_line(y, (self.border_x, self.border_x+5), 1, BLACK);
            if (y as i32 - (self.height - self.border_x) as i32/2).abs() < 20 {
                continue;
            }
            style.render_text(&mut self.pixels, self.border_x, y, &format!("{}", major), BLACK, TextAlign::Center, TextAlign::LowerRight);
        }
        for minor in x_minors {
            let x = self.axis_to_pixel(self.data_to_axis((minor, avg_y)).unwrap()).0.round() as u32;
            self.draw_v_line(x, (self.height - self.border_y, self.height - self.border_y - 2), 1, BLACK);
        }
        for minor in y_minors {
            let y = self.axis_to_pixel(self.data_to_axis((avg_x, minor)).unwrap()).1.round() as u32;
            self.draw_h_line(y, (self.border_x, self.border_x+2), 1, BLACK);
        }
    }

    fn data_to_axis(&self, pos: (f32, f32)) -> Option<(f32, f32)> {
        if pos.0 < self.x_lim.0 || pos.0 > self.x_lim.1 || pos.1 < self.y_lim.0 || pos.1 > self.y_lim.1 {
            return None;
        }
        Some((
            (pos.0 - self.x_lim.0) / (self.x_lim.1 - self.x_lim.0),
            (pos.1 - self.y_lim.0) / (self.y_lim.1 - self.y_lim.0)
        ))
    }

    fn axis_to_pixel(&self, pos: (f32, f32)) -> (f32, f32) {
        (
            self.border_x as f32 + (pos.0 * (self.width - self.border_x) as f32),
            self.height as f32 - (self.border_y as f32 + (pos.1 * (self.height - self.border_y) as f32)),
        )
    }

    /// Draw a circle with center pos = (x, y) in axis coordinates and radius radius (pixels).
    fn draw_circle(&mut self, pos: (f32, f32), radius: f32, color: Rgba<u8>) {
        let radius_i = radius.ceil() as i32 + 1;
        let pos_i = self.axis_to_pixel(pos);
        for i in -radius_i..=radius_i {
            for j in -radius_i..=radius_i {
                let x = i + pos_i.0 as i32;
                let y = j + pos_i.1 as i32;
                if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {continue;}
                let dist2 = (i*i + j*j) as f32;
                if dist2 < (radius - 1.).powi(2) {
                    continue;
                }
                if dist2 > (radius+1.).powi(2) {
                    continue;
                }
                let dist = dist2.sqrt();
                let alpha = 1. - (radius - dist).abs();
                let empty = self.pixels[(x as u32, y as u32)];
                self.pixels[(x as u32, y as u32)] = blend_color(color, empty, alpha);
            }
        }
    }

    /// Draw a filled circle with center pos = (x, y) in axis coordinates and radius radius (pixels).
    fn draw_disk(&mut self, pos: (f32, f32), radius: f32, color: Rgba<u8>) {
        let radius_i = radius.ceil() as i32 + 1;
        let pos_i = self.axis_to_pixel(pos);
        for i in -radius_i..=radius_i {
            for j in -radius_i..=radius_i {
                let x = i + pos_i.0 as i32;
                let y = j + pos_i.1 as i32;
                if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {continue;}
                let dist2 = (i*i + j*j) as f32;
                if dist2 < radius*radius {
                    self.pixels[(x as u32, y as u32)] = color;
                    continue;
                }
                if dist2 > (radius+1.).powi(2) {
                    continue;
                }
                let dist = dist2.sqrt();
                let alpha = radius - dist + 1.;
                let empty = self.pixels[(x as u32, y as u32)];
                self.pixels[(x as u32, y as u32)] = blend_color(color, empty, alpha);
            }
        }
    }

    /// Draw a filled circle with center pos = (x, y) in axis coordinates and radius radius (pixels).
    fn draw_line(&mut self, a: (f32, f32), b: (f32, f32), line_width: f32, color: Rgba<u8>) {
        let half_line_width = line_width / 2.;
        let a_i = self.axis_to_pixel(a);
        let b_i = self.axis_to_pixel(b);
        let v = (b_i.0 as f32 - a_i.0 as f32, b_i.1 as f32 - a_i.1 as f32);
        for i in f32::min(a_i.0, b_i.0).floor() as u32..=f32::max(a_i.0, b_i.0).ceil() as u32 {
            for j in f32::min(a_i.1, b_i.1).floor() as u32..=f32::max(a_i.1, b_i.1).ceil() as u32 {
                let r = (i as f32 - a_i.0 as f32, j as f32 - a_i.1 as f32);
                let dot = r.0*v.0 + r.1*v.1;
                let dist2 = r.0*r.0 + r.1*r.1 - dot*dot / (v.0*v.0 + v.1*v.1);
                if dist2 > half_line_width*half_line_width { continue; }
                let dist = dist2.sqrt();
                let empty = self.pixels[(i, j)];
                self.pixels[(i, j)] = blend_color(empty, color, dist/half_line_width);
            }
        }
    }

    /// Draw a horizontal line with endpoints given by x and thickness line_width in pixels. No antialiasing necessary
    fn draw_h_line(&mut self, y: u32, x: (u32, u32), line_width: u32, color: Rgba<u8>) {
        let half = line_width/2;
        for x in x.0..x.1 {
            for k in 0..line_width {
                self.pixels[(x, y - half + k)] = color;
            }
        }
    }

    /// Draw a vertical line with endpoints given by x and thickness line_width in pixels. No antialiasing necessary
    fn draw_v_line(&mut self, x: u32, y: (u32, u32), line_width: u32, color: Rgba<u8>) {
        let half = line_width/2;
        for y in u32::min(y.0, y.1)..=u32::max(y.0, y.1) {
            if y >= self.height {continue;}
            for k in 0..line_width {
                if x - half + k >= self.width {continue;}
                self.pixels[(x - half + k, y)] = color;
            }
        }
    }
}

impl Element for DynamicPlot {
    fn draw(&self, pixels: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, _style: &Style) {
        // Copy over pixels
        for i in 0..self.pixels.width() {
            for j in 0..self.pixels.height() {
                pixels[(self.top + i, self.left + j)] = self.pixels[(i, j)];
            }
        }
    }

    fn bbox(&self, _mouse: super::Mouse) -> bool {
        false
    }
}