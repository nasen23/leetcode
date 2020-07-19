impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = vec![];
        res.push(nums.clone());
        while next_permutation(&mut nums) {
            res.push(nums.clone());
        }
        res
    }
}

fn next_permutation(nums: &mut Vec<i32>) -> bool {
    let mut i = nums.len().checked_sub(2);
    while let Some(idx) = i {
        if nums[idx] >= nums[idx + 1] {
            i = idx.checked_sub(1);
        } else {
            break;
        }
    }
    if let Some(i) = i {
        let (j, _) = nums
            .iter()
            .enumerate()
            .rev()
            .skip_while(|(_, &x)| x <= nums[i])
            .next()
            .unwrap();
        nums.swap(i, j);
        nums[i + 1..].reverse();
        true
    } else {
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn next_permuate() {
        let mut v = vec![1, 3, 2];
        next_permutation(&mut v);
        assert_eq!(v, vec![2, 1, 3]);
    }

    #[test]
    fn permutations2() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec_vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]]
        );
    }
}
