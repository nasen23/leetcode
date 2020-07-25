impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        if nums.len() == m as usize {
            return nums.into_iter().max().unwrap();
        }
        let (max, sum) = nums
            .iter()
            .fold((0, 0), |(max, sum), &x| (max.max(x), sum + x));
        let (mut l, mut r) = (max, sum);
        while l < r {
            let mid = l + (r - l) / 2;
            let (mut tmp, mut c) = (0, 1);
            for &num in &nums {
                tmp += num;
                if tmp > mid {
                    tmp = num;
                    c += 1;
                }
            }
            if c > m {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_arr_largest_sum() {
        assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(Solution::split_array(vec![1, i32::MAX], 2), i32::MAX);
        assert_eq!(Solution::split_array(vec![1, i32::MAX - 1], 1), i32::MAX);
    }
}
