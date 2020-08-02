impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut top = arr[0];
        let mut c = 0;
        for num in arr.into_iter().skip(1) {
            if num > top {
                top = num;
                c = 1;
            } else {
                c += 1;
            }
            if c >= k {
                return top;
            }
        }
        top
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winner_of_array_game() {
        assert_eq!(Solution::get_winner(vec![1, 25, 35, 42, 68, 70], 1), 25);
        assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
    }
}
