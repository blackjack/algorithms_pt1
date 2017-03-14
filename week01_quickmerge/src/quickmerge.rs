use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct QuickMerge {
    id: Vec<i32>,
}

impl QuickMerge {
    pub fn new(len: usize) -> QuickMerge {
        let mut res = QuickMerge { id: Vec::new() };
        res.id.reserve(len);
        for i in 0..len {
            res.id.push(i as i32);
        }
        res
    }

    pub fn union(&mut self, p: i32, q: i32) {
        let pid = self.root(p) as usize;
        self.id[pid as usize] = self.root(q);
    }

    pub fn root(&self, i: i32) -> i32 {
        let mut idx = i;
        while idx != self.id[idx as usize] {
            idx = self.id[idx as usize];
        }
        idx
    }

    pub fn connected(&self, p: i32, q: i32) -> bool {
        self.root(p) == self.root(q)
    }
}

impl Display for QuickMerge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for i in 0..self.id.len() {
            write!(f, "{:2}", i)?;
        }
        write!(f, "\n")?;
        for i in 0..self.id.len() {
            write!(f, "{:2}", self.id[i])?;
        }
        Ok(())
    }
}


#[test]
fn simple() {
    let mut m = QuickMerge::new(5);
    m.union(1, 2);
    m.union(3, 1);
    assert!(m.root(1) == 2);
    assert!(m.root(3) == 2);
    assert!(m.connected(1, 2));
    assert!(m.connected(1, 3));
}
