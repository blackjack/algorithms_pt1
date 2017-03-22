extern crate rand;

mod quickmerge;
mod percolation;

use percolation::Percolation;
use rand::distributions::{IndependentSample, Range};


fn main() {
    let x = 2000;
    let y = 2000;

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

    let total = percolation.last - 1;
    let open = percolation.number_of_open_sites();
    let ratio = open as f32 / total as f32;
    println!("Total: {}, Open: {}, Ratio: {}", total, open, ratio);
}
