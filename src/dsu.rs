pub struct Dsu {
    parent: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Dsu {
        Dsu {
            parent: (0..size).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let parent = self.find(self.parent[x]);
            self.parent[x] = parent;
            parent
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let (fa_x, fa_y) = (self.find(x), self.find(y));
        if fa_x != fa_y {
            self.parent[fa_x] = fa_y;
        }
    }
}
