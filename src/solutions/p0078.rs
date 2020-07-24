impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mask = (1 << nums.len()) - 1;
        let mut bitmap = 0;
        let mut res = vec![];
        while bitmap <= mask {
            let mut bitmap_it = bitmap;
            let mut tmp = vec![];
            while bitmap_it > 0 {
                let pick: i32 = bitmap_it & -bitmap_it;
                tmp.push(nums[pick.trailing_zeros() as usize]);
                bitmap_it ^= pick;
            }
            res.push(tmp);
            bitmap += 1;
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
    fn subsets() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec_vec![[3], [1], [2], [1, 2, 3], [1, 3], [2, 3], [1, 2], []]
        );
    }
}
