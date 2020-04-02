struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut res = 1;

        let mut min_r = vec![0; arr.len()];
        let mut min = i32::max_value();
        for i in (1..arr.len()).rev() {
            min = std::cmp::min(min, arr[i]);
            min_r[i] = min;
        }

        let mut max = 0;
        for i in 0..arr.len() - 1 {
            max = std::cmp::max(max, arr[i]);
            if max <= min_r[i + 1] {
                res += 1;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]), 4);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 2, 3, 4, 5]), 5);
}
