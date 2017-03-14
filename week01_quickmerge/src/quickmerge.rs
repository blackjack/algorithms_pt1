use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]

pub struct QuickMerge {
    sz: Vec<u32>,
    id: Vec<u32>,
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
            self.sz[i] += self.sz[j];
        }
    }

    pub fn root(&self, i: u32) -> u32 {
        let mut idx = i;
        while idx != self.id[idx as usize] {
            idx = self.id[idx as usize];
        }
        idx
    }

    pub fn connected(&self, p: u32, q: u32) -> bool {
        self.root(p) == self.root(q)
    }
}

impl Display for QuickMerge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for i in 0..self.id.len() {
            write!(f, "{:4}", i)?;
        }
        write!(f, "\n")?;
        for i in 0..self.id.len() {
            write!(f, "[{:2}]", self.id[i])?;
        }
        write!(f, "\n")?;
        for i in 0..self.id.len() {
            write!(f, "{:4}", self.sz[i])?;
        }
        Ok(())
    }
}


#[test]
fn weight_merge() {
    let mut m = QuickMerge::new(10);

    m.union(4, 3);
    m.union(3, 8);
    m.union(6, 5);
    m.union(9, 4);

    assert_eq!(m.root(9), 4);

    m.union(2, 1);
    m.union(5, 0);

    assert_eq!(m.root(0), 6);

    m.union(7, 2);
    m.union(6, 1);

    assert_eq!(m.root(1), 6);
    assert_eq!(m.id[1], 2);

    m.union(7, 3);

    assert_eq!(m.id, vec![6, 2, 6, 4, 6, 6, 6, 2, 4, 4]);
}
