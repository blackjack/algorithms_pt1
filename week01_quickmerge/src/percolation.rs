use std::fmt::{Display, Formatter, Error};
use quickmerge::QuickMerge;

#[allow(dead_code)]
pub struct Percolation {
    x: usize,
    y: usize,
    pub last: usize,
    data: QuickMerge,
}


impl Percolation {
    pub fn new(x: usize, y: usize) -> Percolation {
        let mut p = Percolation {
            x: x,
            y: y,
            last: x * y + 1,
            data: QuickMerge::new(x * y + 2),
        };

        for i in p.data.id.iter_mut() {
            *i = x * y + 2;
        }
        p.data.id[0] = 0;
        p.data.id[p.last] = p.last;
        p
    }

    pub fn index(&self, row: usize, column: usize) -> usize {
        if row == 0 {
            return 0;
        }
        if row > self.y {
            return self.last;
        }

        self.y * (row - 1) + column
    }


    pub fn open(&mut self, row: usize, column: usize) {
        let index = self.index(row, column);

        if !self.is_open(row, column) {
            self.data.id[index] = index;
        }


        let neighbors =
            [(row + 1, column), (row - 1, column), (row, column + 1), (row, column - 1)];

        let x = self.x;

        for &(nrow, ncol) in neighbors.iter()
            .filter(|c| c.1 > 0 && c.1 <= x) {

            if self.is_open(nrow, ncol) {
                let neighbor = self.index(nrow, ncol);
                self.data.union(index, neighbor);
            }

        }

    }

    pub fn is_open(&self, row: usize, column: usize) -> bool {
        let index = self.index(row, column);
        self.data.id[index] <= self.last
    }

    pub fn is_full(&mut self, row: usize, column: usize) -> bool {
        let index = self.index(row, column);
        self.data.connected(index, 0)
    }

    pub fn number_of_open_sites(&self) -> usize {
        let max = self.last;
        let id = &self.data.id[1..max];
        id.iter().filter(|&&i| i < max).count()
    }

    pub fn percolates(&mut self) -> bool {
        let row = self.y + 1;
        self.is_full(row, 1)
    }
}

impl Display for Percolation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {

        write!(f, "{}\n", self.data.id[0])?;

        let print = |row, column| {
            if self.is_open(row, column) {
                let index = self.index(row, column);
                self.data.id[index].to_string()
            } else {
                "X".to_string()
            }
        };
        for row in 1..self.x + 1 {
            for column in 1..self.y + 1 {
                write!(f, "{}", print(row, column))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "{}\n", print(self.y + 1, self.x))
    }
}


#[test]
fn test_open() {
    let mut p = Percolation::new(3, 3);
    p.open(1, 3);
    p.open(2, 2);
    p.open(2, 3);
    p.open(3, 3);
    assert!(p.is_open(4, 3));
    assert!(p.is_open(4, 1));
}

#[test]
fn test_count() {
    let mut p = Percolation::new(3, 3);
    p.open(1, 3);
    p.open(2, 2);
    p.open(2, 3);
    p.open(3, 3);
    assert_eq!(4, p.number_of_open_sites());
}

#[test]
fn test_percolates() {
    let mut p = Percolation::new(3, 3);
    p.open(1, 1);
    p.open(2, 1);
    p.open(2, 2);
    p.open(2, 3);
    assert!(!p.percolates());
    p.open(3, 3);
    assert!(p.percolates());
}
