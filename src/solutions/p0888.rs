impl Solution {
    pub fn fair_candy_swap(mut a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        a.sort_unstable();
        b.sort_unstable();
        let (aa, bb): (i32, i32) = (a.iter().sum(), b.iter().sum());
        let (mut i, mut j) = (0, 0);
        let k = aa - bb;
        while i < a.len() && j < b.len() {
            if 2 * (a[i] - b[j]) == k {
                return vec![a[i], b[j]];
            } else if 2 * (a[i] - b[j]) < k {
                i += 1;
            } else {
                j += 1;
            }
        }
        return vec![];
    }
}

struct Solution;
