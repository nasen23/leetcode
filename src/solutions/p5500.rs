impl Solution {
    pub fn get_max_len(nums: Vec<i32>) -> i32 {
        // a for > 0, b for < 0
        let (mut a, mut b) = (vec![0; nums.len() + 1], vec![0; nums.len() + 1]);
        for (i, n) in nums.into_iter().enumerate() {
            a[i + 1] = if n > 0 {
                a[i] + 1
            } else if n < 0 {
                if b[i] == 0 {
                    0
                } else {
                    b[i] + 1
                }
            } else {
                0
            };
            b[i + 1] = if n > 0 {
                if b[i] == 0 {
                    0
                } else {
                    b[i] + 1
                }
            } else if n < 0 {
                a[i] + 1
            } else {
                0
            };
        }
        a.into_iter().max().unwrap()
    }
}

struct Solution;
