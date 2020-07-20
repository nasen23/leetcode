impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i != j {
            if numbers[i] + numbers[j] == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        unreachable!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum2() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2])
    }
}
