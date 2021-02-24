impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        a.into_iter()
            .map(|x| x.into_iter().rev().map(|x| x ^ 1).collect())
            .collect()
    }
}

struct Solution;
