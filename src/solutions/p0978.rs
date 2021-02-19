use std::cmp::Ordering::*;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 1);
        let mut last = Equal;
        let mut res = 1;
        while j < arr.len() {
            let ordering = arr[j - 1].cmp(&arr[j]);
            if last == ordering || ordering == Equal {
                res = res.max(j - i);
                i = if ordering == Equal { j } else { j - 1 };
            }
            last = ordering;
            j += 1;
        }
        res.max(j - i) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_turbulent_subarray() {
        assert_eq!(
            5,
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9])
        );
        assert_eq!(
            8,
            Solution::max_turbulence_size(vec![0, 8, 45, 88, 48, 68, 28, 55, 17, 24])
        );
        assert_eq!(2, Solution::max_turbulence_size(vec![4, 8, 12, 16]));
        assert_eq!(1, Solution::max_turbulence_size(vec![100]));
    }
}
