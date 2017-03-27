use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct QuickMerge {
    pub sz: Vec<usize>,
    pub id: Vec<usize>,
}

#[allow(dead_code)]
impl QuickMerge {
    pub fn new(len: usize) -> QuickMerge {
        let mut res = QuickMerge {
            id: Vec::new(),
            sz: Vec::new(),
        };
        res.id.reserve(len);
        for i in 0..len {
            res.id.push(i);
        }
        res.sz.resize(len, 1);
        res
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);

        if i == j {
            return;
        }

        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }
    }

    pub fn root(&mut self, i: usize) -> usize {
        let mut idx = i;
        while idx != self.id[idx] {
            let parent_idx = self.id[self.id[idx]];

            // Path compression
            self.id[idx] = parent_idx;
            self.sz[parent_idx] -= self.sz[idx];
            self.sz[self.id[parent_idx]] += self.sz[idx];

            // Move to parent
            idx = self.id[idx];
        }
        idx
    }

    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }
}

impl Display for QuickMerge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Id:     |")?;
        for i in 0..self.id.len() {
            write!(f, "{:3} ", i)?;
        }
        write!(f, "|\nParent: |")?;
        for i in 0..self.id.len() {
            write!(f, "[{:2}]", self.id[i])?;
        }
        write!(f, "|\nSz:     |")?;
        for i in 0..self.id.len() {
            write!(f, "{:3} ", self.sz[i])?;
        }
        write!(f, "|")?;
        Ok(())
    }
}

