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
        let num: [f64; 3] = [0.016819,   0.033638,   0.016819];
        let denom: [f64; 3] = [1.0000, -1.6011,   0.6684];

        for ii in 0..num_rows {
            let first =  self.landscape[ii][0];
            let mut xx:[f64; 3]=[first,first,first];            
            for jj in 0..num_cols {
                let input = self.landscape[ii][jj];
                xx[0] = input - denom[1]*xx[1]  - denom[2]*xx[2];
                let out = num[0]*xx[0] + num[1]*xx[1] + num[2]*xx[2];
                xx[2]=xx[1];
                xx[1]=xx[0];  
                self.landscape[ii][jj]=out;
            }
        }
        
        for ii in 0..num_cols {
            let first =  self.landscape[0][ii];
            let mut xx:[f64; 3]=[first,first,first];            
            for jj in 0..num_rows {
                let input = self.landscape[jj][ii];
                xx[0] = input - denom[1]*xx[1]  - denom[2]*xx[2];
                let out = num[0]*xx[0] + num[1]*xx[1] + num[2]*xx[2];
                xx[2]=xx[1];
                xx[1]=xx[0];  
                self.landscape[jj][ii]=out;
            }
        }              
    }
    
    
    //------------------------------------------------------------
    // plot_landscape
    //------------------------------------------------------------
    pub fn plot_landscape(self) {
        let num_rows = config::NUM_ROWS;
        let num_cols = config::NUM_COLS;
        let mut rows = Vec::with_capacity(num_rows);
        let mut cols = Vec::with_capacity(num_cols);

        for ii in 0..num_rows {
            rows.push(ii as f64);
        }
        for jj in 0..num_cols {
            cols.push(jj as f64);
        }
        let zz = self.landscape;

        // Initialize the plot
        let mut plot = Plot::new();
        // Create the Surface trace
        // Surface::new() expects the Z data to be a Vec<Vec<f64>> (or similar 2D nested structure)
        let trace = Surface::new(zz)
                            .x(rows)
                            .y(cols)
                            .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                            .name("PI landscape");

        plot.add_trace(trace);

        // This will generate an HTML file in your temp directory and open it in your default browser
        plot.show();
    }
}
