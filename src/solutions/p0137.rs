struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bits = [0i32; 32];
        let mut res = 0i32;

        for num in nums {
            for i in 0..32 {
                bits[i] += (num >> i) & 1;
            }
        }

        for i in 0..32 {
            if bits[i] % 3 == 1 {
                res = res.wrapping_add(2i32.checked_pow(i as u32).unwrap_or(i32::min_value()));
            }
        }

        res
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::single_number(vec![1, 1, 1, 2]), 2);
    assert_eq!(Solution::single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    assert_eq!(
        Solution::single_number(vec![-2, -2, 1, 1, -3, 1, -3, -3, -4, -2]),
        -4
    );
}
