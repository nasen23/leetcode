impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut res = vec![];
        let mut nums = nums;
        nums.sort_unstable();

        for z in 0..nums.len() - 3 {
            if z > 0 && nums[z] == nums[z - 1] {
                continue;
            }
            if nums[z] + 3 * nums[z + 1] > target {
                break;
            }
            if nums[z] + 3 * nums[nums.len() - 1] < target {
                continue;
            }
            let sum3 = target - nums[z];
            for k in z + 1..nums.len() - 2 {
                if k > z + 1 && nums[k] == nums[k - 1] {
                    continue;
                }
                let sum2 = sum3 - nums[k];
                let (mut i, mut j) = (k + 1, nums.len() - 1);
                while i < j {
                    if nums[i] + nums[j] == sum2 {
                        res.push(vec![nums[z], nums[k], nums[i], nums[j]]);
                        while i < j && nums[i] == nums[i + 1] {
                            i += 1;
                        }
                        while i < j && nums[j] == nums[j - 1] {
                            j -= 1;
                        }
                        i += 1;
                        j -= 1;
                    } else if nums[i] + nums[j] < sum2 {
                        i += 1;
                    } else {
                        j -= 1;
                    }
                }
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn test_name() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec_vec![[-1, 0, 0, 1], [-2, -1, 1, 2], [-2, 0, 0, 2]]
        );
    }
}
