use quickmerge::QuickMerge;
use std::collections::HashMap;
use std::cmp;

pub struct SocialGraph {
    pub m: QuickMerge,
    pub max_element: HashMap<usize, usize>
}

impl SocialGraph {
    pub fn new(len: usize) -> SocialGraph {
        SocialGraph {
            m: QuickMerge::new(len),
            max_element: HashMap::new(),
        }
    }

    pub fn connect(&mut self, user1: usize, user2: usize) -> bool {
        self.m.union(user1, user2);

        let root = self.m.root(user1);

        let mut max = self.max_element.entry(root).or_insert(0);
        *max = cmp::max(*max, user1);
        *max = cmp::max(*max, user2);

        self.m.sz[root] == self.m.sz.len()
    }

    pub fn max(&mut self, e: usize) -> usize {
        let root = self.m.root(e);
        *self.max_element.entry(root).or_insert(0)
    }
}
