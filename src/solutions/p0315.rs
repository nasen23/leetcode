use std::collections::HashMap;

#[inline]
fn lowbit(x: usize) -> usize {
    x & (!x + 1)
}

struct BinaryIndexTree {
    inner: Vec<i32>,
}

impl BinaryIndexTree {
    fn new(nums: &[i32]) -> Self {
        let mut inner = Vec::with_capacity(nums.len());
        for &x in nums {
            inner.push(x);
        }
        for i in 0..inner.len() {
            let j = i + lowbit(i + 1);
            if j < inner.len() {
                inner[j] += inner[i];
            }
        }
        Self { inner }
    }

    fn update(&mut self, i: usize, d: i32) {
        let mut i = i;
        while i < self.inner.len() {
            self.inner[i] += d;
            i += lowbit(i + 1);
        }
    }

    fn set(&mut self, i: usize, val: i32) {
        self.update(i, val - self.inner[i]);
    }

    fn prefix_sum(&self, i: usize) -> i32 {
        // prefix sum [0, i)
        let mut i = match i.checked_sub(1) {
            Some(i) => i,
            None => return 0,
        };
        let mut res = 0;
        loop {
            res += self.inner[i];
            match i.checked_sub(lowbit(i + 1)) {
                Some(x) => i = x,
                None => break res,
            }
        }
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut bit = BinaryIndexTree::new(&vec![0; nums.len()]);
        let mut x = nums.clone();
        x.sort();
        x.dedup();
        let map = x
            .into_iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();
        let mut res = nums
            .into_iter()
            .rev()
            .map(|n| {
                let &i = map.get(&n).unwrap();
                bit.update(i, 1);
                bit.prefix_sum(i)
            })
            .collect::<Vec<_>>();
        res.reverse();
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_smaller_after_self() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    }

    #[test]
    fn binary_index_tree() {
        let mut bit = BinaryIndexTree::new(&[1, 2, 3, 4]);
        assert_eq!(bit.prefix_sum(1), 3);
        bit.set(0, 10);
        assert_eq!(bit.prefix_sum(0), 10);
    }
}
