use crate::config;
use plotly::common::{ColorScale, ColorScalePalette};
use plotly::{Plot, Surface};

pub struct Landscape {
    pub landscape: Vec<Vec<f64>>,
}

impl Landscape {
    //------------------------------------------------------------
    // Constructor
    //------------------------------------------------------------
    pub fn new(digits: Vec<i32>) -> Self {
        let num_rows = config::NUM_ROWS;
        let num_cols = config::NUM_COLS;
        let mut landscape: Vec<Vec<f64>> = vec![vec![0.0; num_cols]; num_rows];
        for ii in 0..num_rows {
            for jj in 0..num_cols {
                let index = (ii * num_cols) + jj;
                landscape[ii][jj] = digits[index] as f64;
            }
        }
        Landscape { landscape }
    }

    //------------------------------------------------------------
    // filter_landscape
    //------------------------------------------------------------
    pub fn filter_landscape(&mut self) {
        let num_rows = config::NUM_ROWS;
        let num_cols = config::NUM_COLS;
        let num: [f64; 3] = [0.034786,   0.069572,   0.034786];
        let denom: [f64; 3] = [1.0000, -1.4075,   0.5466];

        for ii in 0..num_rows {
            let first = self.landscape[ii][0];
            let mut xx: [f64; 3] = [first, first, first];
            for jj in 0..num_cols {
                let input = self.landscape[ii][jj];
                xx[0] = input - denom[1] * xx[1] - denom[2] * xx[2];
                let out = num[0] * xx[0] + num[1] * xx[1] + num[2] * xx[2];
                xx[2] = xx[1];
                xx[1] = xx[0];
                self.landscape[ii][jj] = out;
            }
        }

        for ii in 0..num_cols {
            let first = self.landscape[0][ii];
            let mut xx: [f64; 3] = [first, first, first];
            for jj in 0..num_rows {
                let input = self.landscape[jj][ii];
                xx[0] = input - denom[1] * xx[1] - denom[2] * xx[2];
                let out = num[0] * xx[0] + num[1] * xx[1] + num[2] * xx[2];
                xx[2] = xx[1];
                xx[1] = xx[0];
                self.landscape[jj][ii] = out;
            }
        }
    }

    //------------------------------------------------------------
    // plot_landscape
    //------------------------------------------------------------
    pub fn plot_landscape(self) {
        let num_rows = config::NUM_ROWS;
        let num_cols = config::NUM_COLS;
        let mut xs: Vec<Vec<f64>> = vec![vec![0.0; num_cols]; num_rows];
        let mut ys: Vec<Vec<f64>> = vec![vec![0.0; num_cols]; num_rows];
        let mut zs: Vec<Vec<f64>> = vec![vec![0.0; num_cols]; num_rows];

        let r_base = 10.0;
        for ii in 0..num_rows {
            for jj in 0..num_cols {
                let theta = std::f64::consts::PI * ii as f64 / num_rows as f64;
                let phi = 2.0 * std::f64::consts::PI * jj as f64 / num_cols as f64;
                let r = r_base + self.landscape[ii][jj];
                xs[ii][jj] = r * theta.sin() * phi.cos();
                ys[ii][jj] = r * theta.sin() * phi.sin();
                zs[ii][jj] = r * theta.cos();
            }
        }

        // Initialize the plot
        let mut plot = Plot::new();

        let trace = Surface::<Vec<f64>, Vec<f64>, f64>::new(zs)
            .x(xs)
            .y(ys)
            .color_scale(ColorScale::Palette(ColorScalePalette::Rainbow))
            .name("PI landscape");
        plot.add_trace(trace);

        plot.show();
    }
}
