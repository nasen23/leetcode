impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        let mut res = vec![];
        for i in 0..nums.len() - 2 {
            if i == 0 || (i > 0 && nums[i] != nums[i - 1]) {
                let (mut l, mut r) = (i + 1, nums.len() - 1);
                while l < r {
                    let s = nums[i] + nums[l] + nums[r];
                    if s < 0 {
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        l += 1;
                    } else if s > 0 {
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        r -= 1;
                    } else {
                        res.push(vec![nums[i], nums[l], nums[r]]);
                        while l < r && nums[l] == nums[l + 1] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
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
    use super::Solution;
    use crate::vec_vec;

    #[test]
    fn three_sum() {
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec_vec![[-1, 0, 1], [-1, -1, 2]]
        )
    }
}
