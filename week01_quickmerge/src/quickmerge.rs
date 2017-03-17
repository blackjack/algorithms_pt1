use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct QuickMerge {
    pub sz: Vec<usize>,
    pub id: Vec<u32>,
}

impl QuickMerge {
    pub fn new(len: usize) -> QuickMerge {
        let mut res = QuickMerge {
            id: Vec::new(),
            sz: Vec::new(),
        };
        res.id.reserve(len);
        for i in 0..len {
            res.id.push(i as u32);
        }
        res.sz.resize(len, 1);
        res
    }

    pub fn union(&mut self, p: u32, q: u32) {
        let i = self.root(p) as usize;
        let j = self.root(q) as usize;

        if i == j {
            return;
        }

        if self.sz[i] < self.sz[j] {
            self.id[i] = j as u32;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i as u32;
            print!("Sz[{}]={}\n",i,self.sz[i]);
            print!("Sz[{}]={}\n",j,self.sz[j]);
            self.sz[i] += self.sz[j];
        }
    }

    pub fn root(&mut self, i: u32) -> u32 {
        let mut idx = i as usize;
        while idx != self.id[idx] as usize {
            let parent_idx = self.id[self.id[idx] as usize];

            // Path compression
            self.id[idx] = parent_idx;
            self.sz[parent_idx as usize] -= self.sz[idx];
            self.sz[self.id[parent_idx as usize] as usize] += self.sz[idx];

            // Move to parent
            idx = self.id[idx] as usize;
        }
        idx as u32
    }

    pub fn connected(&mut self, p: u32, q: u32) -> bool {
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

