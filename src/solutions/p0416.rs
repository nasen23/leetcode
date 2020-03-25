struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 != 0 {
            return false;
        }

        let target = sum / 2;
        let mut dp = vec![false; target + 1];
        if nums[0] > target as i32 {
            return false;
        }
        dp[nums[0] as usize] = true;

        for i in 1..nums.len() {
            let index = target as i32 - nums[i];
            if index > 0 && dp[index as usize] {
                return true;
            }
            for j in (1..target).rev() {
                if j < nums[i] as usize {
                    break;
                }
                dp[j] = dp[j] || dp[j - nums[i] as usize];
            }
        }

        dp[target]
    }
}

#[test]
fn case1() {
    let nums = vec![1, 5, 11, 5];
    assert_eq!(Solution::can_partition(nums), true);
}

#[test]
fn case2() {
    let nums = vec![1, 2, 3, 5];
    assert_eq!(Solution::can_partition(nums), false);
}

#[test]
fn case3() {
    let nums = vec![1, 2, 5];
    assert_eq!(Solution::can_partition(nums), false);
}

#[test]
fn case4() {
    let nums = vec![100];
    assert_eq!(Solution::can_partition(nums), false);
}
