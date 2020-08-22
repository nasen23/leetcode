impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        judge_point24_impl(nums.into_iter().map(|x| x as f64).collect())
    }
}

fn judge_point24_impl(nums: Vec<f64>) -> bool {
    if nums.len() == 1 {
        return (nums[0] - 24.0).abs() < 1e-5;
    }
    macro_rules! copy_skip {
        ($v:expr, $i:expr, $j:expr, $x:expr) => {{
            let mut new = Vec::with_capacity($v.len() - 1);
            for (k, &x) in $v.iter().enumerate() {
                if k != $i && k != $j {
                    new.push(x);
                }
            }
            new.push($x);
            new
        }};
    }
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if judge_point24_impl(copy_skip!(nums, i, j, nums[i] + nums[j])) {
                return true;
            }
            if judge_point24_impl(copy_skip!(nums, i, j, nums[i] - nums[j])) {
                return true;
            }
            if judge_point24_impl(copy_skip!(nums, i, j, nums[j] - nums[i])) {
                return true;
            }
            if judge_point24_impl(copy_skip!(nums, i, j, nums[i] * nums[j])) {
                return true;
            }
            if judge_point24_impl(copy_skip!(nums, i, j, nums[i] / nums[j])) {
                return true;
            }
            if judge_point24_impl(copy_skip!(nums, i, j, nums[j] / nums[i])) {
                return true;
            }
        }
    }
    false
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twentyfour_game() {
        println!("{}", 6.0 / (1.0 - 3.0 / 4.0));
        println!("{}", 8.0 / (3.0 - 8.0 / 3.0));
        assert!(Solution::judge_point24(vec![4, 1, 8, 7]));
        assert!(Solution::judge_point24(vec![1, 3, 4, 6]));
        assert!(Solution::judge_point24(vec![3, 3, 8, 8]));
        assert!(!Solution::judge_point24(vec![1, 1, 7, 7]));
        assert!(!Solution::judge_point24(vec![1, 2, 1, 2]));
    }
}
