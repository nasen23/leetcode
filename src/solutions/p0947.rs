use std::collections::HashSet;

struct UFS(Vec<usize>);

impl UFS {
    fn new(n: usize) -> Self {
        Self((0..n).collect())
    }

    fn find(&mut self, x: usize) -> usize {
        if self.0[x] != x {
            self.0[x] = self.find(self.0[x]);
        }
        self.0[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        self.0[px] = self.0[py];
    }
}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let n = stones.len();
        if n == 0 {
            return 0;
        }
        let mut ufs = UFS::new(20000);
        for (x, y) in stones.iter().map(|v| (v[0] as usize, v[1] as usize)) {
            ufs.union(x, y + 10000);
        }
        (n - stones
            .into_iter()
            .map(|v| ufs.find(v[0] as usize))
            .collect::<HashSet<_>>()
            .len()) as i32
    }
}

struct Solution;
