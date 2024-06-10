use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
//wasm-pack build --target web
use wasm_bindgen::prelude::*;

use wikid_wasm::{log, Applet, Callback, Dim, Style, TextAlign};
use wikid_wasm::element::{Button, DynamicPlot, PlotCommand, Element, Slider, SliderType, LineStyle};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[wasm_bindgen]
pub struct LinearFitDemo {
    applet: Applet,
    a_slider: Slider,
    b_slider: Slider,
    sigma_slider: Slider,
    n_slider: Slider,
    randomize_button: Button,
    plot: DynamicPlot,

    seed: u64,
    n_successes: u32,
    n_trials: u32,
    just_refreshed_seed: bool,
    success_symbol: char,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl LinearFitDemo {
    pub fn new(canvas: String) -> Self {
        wikid_wasm::debug_panic();

        let mut style = Style::default(include_bytes!("../../../../fonts/cmunrm.ttf"));
        style.set_color("#eca500");
        let applet = Applet::new(WIDTH, HEIGHT, canvas, style);

        let a_slider = Slider::new(Dim::Percent(0.18), Dim::Percent(0.075), "a".to_owned(), SliderType::Float(2), [-2., 2., 0.], WIDTH, HEIGHT);

        let b_slider = Slider::new(Dim::Percent(0.18), Dim::Percent(0.125), "b".to_owned(), SliderType::Float(2), [-1., 1., 0.], WIDTH, HEIGHT);

        let sigma_slider = Slider::new(Dim::Percent(0.53), Dim::Percent(0.075), "σ".to_owned(), SliderType::Float(2), [0.001, 2., 1.], WIDTH, HEIGHT);

        let n_slider = Slider::new(Dim::Percent(0.53), Dim::Percent(0.125), "N".to_owned(), SliderType::Int, [2., 50., 10.], WIDTH, HEIGHT);

        let randomize_button = Button::new((Dim::Percent(0.85), Dim::Percent(0.1)), "Randomize".to_owned(), WIDTH, HEIGHT);

        let plot = DynamicPlot::new((
            Dim::Percent(0.025),
            Dim::Percent(0.2),
            Dim::Percent(0.95),
            Dim::Percent(0.8)),
            WIDTH, HEIGHT
        );

        Self {
            applet,
            a_slider,
            b_slider,
            sigma_slider,
            n_slider,
            randomize_button,
            plot,
            seed: 423894187419,
            n_successes: 0,
            n_trials: 0,
            just_refreshed_seed: true,
            success_symbol: '0',
        }
    }

    pub fn render(&mut self) {
        self.applet.render(&self.get_elements());
    }

    pub fn tick(&mut self) {
        // let mut rng = rand::thread_rng();
        let mut rng = rand::rngs::SmallRng::seed_from_u64(self.seed);
        let n = self.n_slider.get_value() as usize;
        let sigma = self.sigma_slider.get_value();

        let mut xs = Vec::with_capacity(n);
        let mut true_ys = Vec::with_capacity(n);
        let mut ys = Vec::with_capacity(n);
        let normal = Normal::new(0., sigma).unwrap();

        let a = self.a_slider.get_value();
        let b = self.b_slider.get_value();
        for i in 0..n {
            let x = (i as f32) / (n as f32 - 1.);
            let true_y = a * x + b;
            let y = normal.sample(&mut rng) + true_y;
            xs.push(x);
            true_ys.push(true_y);
            ys.push(y);
        }

        let mut _x_ = 0.;
        let mut _y_ = 0.;
        let mut _xx_ = 0.;
        let mut _xy_ = 0.;
        for (x, y) in xs.iter().zip(&ys) {
            _x_ += *x;
            _y_ += *y;
            _xx_ += *x**x;
            _xy_ += *x**y;
        }
        _x_ /= n as f32;
        _y_ /= n as f32;
        _xx_ /= n as f32;
        _xy_ /= n as f32;
        let var = _xx_ - _x_*_x_;
        let cov = _xy_ - _x_*_y_;
        let b_num = _xx_*_y_ - _x_*_xy_;
        
        let a_bf = cov / var;
        let a_unc = sigma / (n as f32 * var).sqrt();
        let b_bf = b_num / var;
        let b_unc = sigma * _xx_.sqrt() / (n as f32 * var).sqrt();
        let cov = -sigma*sigma * _x_ / n as f32 / var;
        let mut bf_ys = Vec::with_capacity(n);
        let mut y_err_low = Vec::with_capacity(n);
        let mut y_err_high = Vec::with_capacity(n);
        for x in &xs {
            let bf_y = a_bf * x + b_bf;
            bf_ys.push(bf_y);
            let y_err = (2.3 * (x*x*a_unc*a_unc + b_unc*b_unc + 2.*x*cov)).sqrt();
            y_err_low.push(bf_y - y_err);
            y_err_high.push(bf_y + y_err);
        }

        if self.just_refreshed_seed {
            let sigmai_11 = n as f32 / (sigma*sigma) * _xx_;
            let sigmai_22 = n as f32 / (sigma*sigma);
            let sigmai_12 = n as f32 / (sigma*sigma) * _x_;
            let sigma_dot = (a_bf - a).powi(2) * sigmai_11 + (b_bf - b).powi(2) * sigmai_22 + 2. * (a_bf - a) * (b_bf - b) * sigmai_12;
            if sigma_dot < 2.3 {
                self.n_successes += 1;
                self.success_symbol = 'Y';
            } else {
                self.success_symbol = 'N';
            }
            self.n_trials += 1;
            self.just_refreshed_seed = false;
        }

        let f_bf = self.n_successes as f32 / self.n_trials as f32;
        let f_unc = (0.68 * (1.-0.68) / (self.n_trials as f32)).sqrt();

        self.plot.plot(vec![
            PlotCommand::Line{xs: &xs, ys: &true_ys, ls: LineStyle::Dashed },
            PlotCommand::FillBetween { xs: &xs, y1s: &y_err_low, y2s: &y_err_high },
            PlotCommand::Line{xs: &xs, ys: &bf_ys, ls: LineStyle::Solid },
            PlotCommand::ErrorBar { xs: &xs, ys: &ys, y_errs: &vec![sigma; xs.len()] },
            PlotCommand::Scatter{xs: &xs, ys: &ys },
            PlotCommand::SetXLabel { label: "x".to_owned() },
            PlotCommand::SetYLabel { label: "y".to_owned() },
            PlotCommand::Text { x: 0.03, y: 1., text: format!("a = {:.2} ± {:.2}", a_bf, a_unc), va: TextAlign::UpperLeft, ha: TextAlign::UpperLeft },
            PlotCommand::Text { x: 0.03, y: 0.93, text: format!("b = {:.2} ± {:.2}", b_bf, b_unc), va: TextAlign::UpperLeft, ha: TextAlign::UpperLeft },
            PlotCommand::Text { x: 0.03, y: 0.07, text: format!("% correct = {:.0} ± {:.0} ({})", f_bf*100., f_unc*100., self.success_symbol), va: TextAlign::UpperLeft, ha: TextAlign::UpperLeft },
        ], &self.applet.style);
        let elements = self.get_mut_elements();
        for callback in self.applet.tick(elements) {
            match callback {
                Callback::ButtonClicked(b) => {
                    if b as *const Button == &self.randomize_button as *const Button {
                        self.seed = rand::thread_rng().gen();
                        self.just_refreshed_seed = true;
                    }
                }
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

impl LinearFitDemo {
    fn get_elements(&self) -> Vec<*const dyn Element> {
        vec![
            &self.a_slider,
            &self.b_slider,
            &self.sigma_slider,
            &self.n_slider,
            &self.randomize_button,
            &self.plot,
        ]
    }
    fn get_mut_elements(&mut self) -> Vec<*mut dyn Element> {
        vec![
            &mut self.a_slider,
            &mut self.b_slider,
            &mut self.sigma_slider,
            &mut self.n_slider,
            &mut self.randomize_button,
            &mut self.plot,
        ]
    }
}