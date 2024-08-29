struct UnionFind {
    p: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        let mut size = vec![1; n];
        for i in 0..n {
            p[i] = i;
        }
        UnionFind { p, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            return false;
        }
        if self.size[pa] > self.size[pb] {
            self.p[pb] = pa;
            self.size[pa] += self.size[pb];
        } else {
            self.p[pa] = pb;
            self.size[pb] += self.size[pa];
        }
        true
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let m = 10001;
        let mut uf = UnionFind::new(m * 2);
        for st in &stones {
            uf.union(st[0] as usize, (st[1] + m as i32) as usize);
        }
        let mut s = std::collections::HashSet::new();
        for st in &stones {
            s.insert(uf.find(st[0] as usize));
        }
        (stones.len() - s.len()) as i32
    }
}