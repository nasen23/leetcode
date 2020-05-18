impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let mut digits = vec![];
        let mut m = n;
        while m > 0 {
            digits.push(m % 10);
            m /= 10;
        }
        let len = digits.len();

        let mut used = [false; 10];
        let mut res = 0;

        for i in 1..len {
            res += 9 * permutations(9, i as i32 - 1);
        }

        for (i, num) in digits.into_iter().enumerate().rev() {
            for j in if i == len - 1 { 1 } else { 0 }..num {
                if unsafe { *used.get_unchecked(j as usize) } {
                    continue;
                }
                res += permutations(10 - (len - i) as i32, i as i32);
            }

            let mut_ref = unsafe { used.get_unchecked_mut(num as usize) };
            if *mut_ref {
                break;
            }
            *mut_ref = true;

            if i == 0 {
                res += 1;
            }
        }

        n - res
    }
}

#[inline]
fn permutations(m: i32, n: i32) -> i32 {
    (m - n + 1..=m).fold(1, |acc, x| acc * x)
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::{permutations, Solution};

    #[test]
    fn permutation_test() {
        assert_eq!(permutations(10, 1), 10);
        assert_eq!(permutations(10, 2), 90);
    }

    #[test]
    fn num_dup_digits_at_most_n() {
        assert_eq!(Solution::num_dup_digits_at_most_n(1000), 262);
    }
}
