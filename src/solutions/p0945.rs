struct Solution;

impl Solution {
    pub fn min_increment_for_unique(a: Vec<i32>) -> i32 {
        let mut a = a;
        let mut op = 0;
        a.sort();

        for i in 1..a.len() {
            if a[i] <= a[i - 1] {
                let movep = a[i - 1] - a[i] + 1;
                op += movep;
                a[i] += movep;
            }
        }

        op
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
}

#[test]
fn case2() {
    assert_eq!(
        Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]),
        6
    );
}
