//Compile with `wasm-pack build --target web`
use wasm_bindgen::prelude::*;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use wikid_wasm::{log, Applet, Dim, Style};
use wikid_wasm::element::{DynamicPlot, Element, LineStyle, PlotCommand, Slider, SliderType};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[wasm_bindgen]
pub struct MhDemo {
    applet: Applet,
    plot: DynamicPlot,
    speed: Slider,

    plot_xs: Vec<f32>,
    true_ys: Vec<f32>,
    progress_index: i32,

    trial_x: f64,
    walker_x: f64,
    hist_edges: Vec<f32>,
    hist_counts: Vec<usize>,
    accept: bool,
    rng: SmallRng,
}

fn like(x: f64) -> f64 {
    30. * ((x-0.5).powi(2) - 4. * (x - 0.5).powi(4))
}

#[wasm_bindgen]
impl MhDemo {
    pub fn new(canvas: String) -> Self {
        wikid_wasm::debug_panic();

        let mut style = Style::default(include_bytes!("../../../../fonts/cmunrm.ttf"));
        style.set_color("#eca500");
        let applet = Applet::new(WIDTH, HEIGHT, canvas, style);

        let plot = DynamicPlot::new((
            Dim::Percent(0.01),
            Dim::Percent(0.1),
            Dim::Percent(0.98),
            Dim::Percent(0.88)),
            WIDTH, HEIGHT
    );

        let speed = Slider::new(Dim::Percent(0.5), Dim::Percent(0.05), "Speed".to_owned(), SliderType::Int, [1., 100., 5.], WIDTH, HEIGHT);

        let plot_xs = linspace(0., 1., 100).into_iter().map(|v| v as f32).collect::<Vec<_>>();
        let true_ys = plot_xs.iter().map(|x| like(*x as f64) as f32).collect::<Vec<_>>();

        let rng = SmallRng::from_entropy();
        let hist_edges = linspace(0., 1., 21).into_iter().map(|v| v as f32).collect::<Vec<_>>();
        let hist_counts = vec![0; hist_edges.len()];

        Self {
            applet,
            plot,
            speed,

            plot_xs,
            true_ys,
            progress_index: -1,

            hist_edges,
            hist_counts,

            trial_x: 0.,
            walker_x: 0.,
            accept: false,
            rng,
        }
    }

    pub fn render(&mut self) {
        self.applet.render(&self.get_elements());
    }

    pub fn tick(&mut self) {
        let frames_per_tick = (100. / self.speed.get_value() as f64) as i32;
        if self.progress_index % frames_per_tick == 0 {
            match self.progress_index / frames_per_tick {
                0 => {
                    // Sample a new point
                    self.trial_x = self.rng.gen::<f64>();
                },
                1 => {
                    // Reject or accept the point
                    if like(self.walker_x) < like(self.trial_x) {
                        // Accept
                        self.accept = true;
                    } else {
                        if self.rng.gen::<f64>() < like(self.trial_x) / like(self.walker_x) {
                            self.accept = true;
                        } else {
                            self.accept = false;
                        }
                    }
                },
                2 => {
                    // Add the point to the chain
                    if self.accept {
                        let index = (self.trial_x as f32 / (self.hist_edges[1] - self.hist_edges[0])) as usize;
                        self.hist_counts[index] += 1;

                        self.walker_x = self.trial_x;
                    }
                },
                _ => self.progress_index = -1,
            };
        }

        let dx = self.hist_edges[1] - self.hist_edges[0];
        let integral = self.hist_counts.iter().fold(0f32, |accum, c| accum + *c as f32) * dx;
        let hist_display = self.hist_counts.iter().map(|c| *c as f32 / integral ).collect::<Vec<_>>();


        self.plot.plot(vec![
            PlotCommand::Line{xs: &self.plot_xs, ys: &self.true_ys, ls: LineStyle::Dashed},
            PlotCommand::Bar{edges: &self.hist_edges, ys: &hist_display},
            PlotCommand::Scatter { xs: &[self.trial_x as f32], ys: &[like(self.trial_x) as f32] }
        ], &self.applet.style);
        self.progress_index += 1;

        let elements = self.get_mut_elements();
        for callback in self.applet.tick(elements) {
            match callback {
                _ => ()
            }
        }
    }

    pub fn mouse_button_down(&mut self, x: u32, y: u32) {
        let elements = self.get_mut_elements();
        self.applet.mouse_button_down(x, y, elements);
    }

    pub fn mouse_button_up(&mut self, x: u32, y: u32) {
        let elements = self.get_mut_elements();
        self.applet.mouse_button_up(x, y, elements);
    }

    pub fn mouse_move(&mut self, x: u32, y: u32) {
        let elements = self.get_mut_elements();
        self.applet.mouse_move(x, y, elements);
    }
}

fn linspace(start: f64, stop: f64, n: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push((stop - start) * i as f64 / (n as f64- 1.) + start);
    }
    out
}

impl MhDemo {
    fn get_elements(&self) -> Vec<*const dyn Element> {
        vec![
            &self.speed,
            &self.plot,
        ]
    }
    fn get_mut_elements(&mut self) -> Vec<*mut dyn Element> {
        vec![
            &mut self.speed,
            &mut self.plot,
        ]
    }
}