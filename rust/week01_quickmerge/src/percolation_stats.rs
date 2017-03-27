use percolation::Percolation;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::fmt::{Display, Formatter, Error};

#[allow(dead_code)]
pub struct PercolationStats {
    samples: Vec<f64>,
}

fn percolate(x: usize, y: usize) -> usize {
    let mut percolation = Percolation::new(x, y);

    let between = Range::new(1, x * y);
    let mut rng = rand::thread_rng();

    while {
        let sample = between.ind_sample(&mut rng);
        let row = sample / y + 1;
        let column = sample % y + 1;
        percolation.open(row, column);

        !percolation.percolates()
    } {}

    percolation.number_of_open_sites()
}


impl PercolationStats {
    pub fn new(grid_size: usize, trials: usize) -> PercolationStats {
        let mut p = PercolationStats { samples: Vec::new() };
        p.samples.reserve_exact(trials);

        let total = grid_size * grid_size;
        for _ in 1..trials + 1 {
            let open = percolate(grid_size, grid_size);
            let ratio = open as f64 / total as f64;
            p.samples.push(ratio);
        }

        p
    }

    pub fn mean(&self) -> f64 {
        let sum: f64 = self.samples.iter().sum();
        let t = self.samples.len() as f64;
        sum / t
    }

    pub fn stddev(&self, mean: f64) -> f64 {
        let dev: f64 = self.samples.iter().map(|s| s - mean).sum();
        let t = (self.samples.len() - 1) as f64;
        dev / t
    }

    pub fn confidence(&self, mean: f64, stddev: f64) -> (f64, f64) {
        let st = (self.samples.len() as f64).sqrt();
        (mean - (1.96 * stddev / st), mean + (1.96 * stddev / st))
    }
}


impl Display for PercolationStats {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        let mean = self.mean();
        let stddev = self.stddev(mean);
        let confidence = self.confidence(mean, stddev);

        write!(f,
               "\
mean                    = {}\n\
stddev                  = {}\n\
95% confidence interval = [{}, {}]" ,
               mean,
               stddev,
               confidence.0,
               confidence.1)
    }
}
