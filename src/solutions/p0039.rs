impl Solution {
    pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];
        combination_sum_impl(&nums, &mut vec![], &mut res, target, 0);
        res
    }
}

fn combination_sum_impl(
    nums: &Vec<i32>,
    tmp: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    t: i32,
    i: usize,
) {
    if t == 0 {
        res.push(tmp.clone());
        return;
    }
    if t < nums[i] {
        return;
    }
    for i in (i..nums.len()).take_while(|&i| nums[i] <= t) {
        tmp.push(nums[i]);
        combination_sum_impl(nums, tmp, res, t - nums[i], i);
        tmp.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn combination_sum() {
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 6, 7], 7),
        //     vec_vec![[7], [2, 2, 3]]
        // );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec_vec![[2, 2, 2, 2], [2, 3, 3], [3, 5]]
        );
    }
}
