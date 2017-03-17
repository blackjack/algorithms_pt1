mod quickmerge;
use quickmerge::QuickMerge;

struct SocialGraph {
    m: QuickMerge,
}

impl SocialGraph {
    fn new(len: usize) -> SocialGraph {
        SocialGraph { m: QuickMerge::new(len) }
    }

    fn connect(&mut self, user1: u32, user2: u32) -> bool {
        self.m.union(user1, user2);

        let root = self.m.root(user1) as usize;
        return self.m.sz[root] == self.m.sz.len();
    }
}




fn num_roots() {
    let mut m = SocialGraph::new(3);

    print!("{}\n", m.connect(0, 1));
    print!("{}\n", m.m);
    print!("{}\n", m.connect(1, 2));
    print!("{}\n", m.m);
}


fn main() {
    num_roots();
}
