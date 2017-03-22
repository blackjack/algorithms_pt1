extern crate rand;

mod quickmerge;
mod percolation;
mod percolation_stats;

use percolation_stats::PercolationStats;

fn main() {
    let mut args = std::env::args();
    args.next();
    let grid: usize = args.next().unwrap().parse().unwrap();
    let times: usize = args.next().unwrap().parse().unwrap();

    let stats = PercolationStats::new(grid, times);
    println!("{}", stats);
}
