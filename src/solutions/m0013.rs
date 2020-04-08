struct Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let mut solver = Solver::new(m, n, k);
        solver.dfs(0, 0)
    }
}

struct Solver {
    m: i32,
    n: i32,
    k: i32,
    visited: Vec<Vec<bool>>,
}

impl Solver {
    fn new(m: i32, n: i32, k: i32) -> Self {
        Self {
            m,
            n,
            k,
            visited: vec![vec![false; n as usize]; m as usize],
        }
    }

    fn dfs(&mut self, i: i32, j: i32) -> i32 {
        if i < 0
            || j < 0
            || i >= self.m
            || j >= self.n
            || num_digits_sum(i) + num_digits_sum(j) > self.k
            || self.visited[i as usize][j as usize]
        {
            return 0;
        }

        self.visited[i as usize][j as usize] = true;

        1 + self.dfs(i - 1, j) + self.dfs(i, j - 1) + self.dfs(i + 1, j) + self.dfs(i, j + 1)
    }
}

#[inline]
fn num_digits_sum(n: i32) -> i32 {
    let mut n = n;
    let mut res = 0;
    while n != 0 {
        res += n % 10;
        n /= 10;
    }

    res
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::moving_count(2, 3, 1), 3);
    }
}
