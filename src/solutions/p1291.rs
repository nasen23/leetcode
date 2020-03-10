struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut nums = vec![];

        let low_bits = low.to_string().len() as i32;
        let high_bits = high.to_string().len() as i32;

        for c in low_bits..=high_bits {
            for i in 1..=10 - c {
                let num: i32 = (i..i + c)
                    .enumerate()
                    .map(|(i, x)| x * 10i32.pow((c - i as i32 - 1) as u32))
                    .sum();
                if num >= low && num <= high {
                    nums.push(num);
                }
            }
        }

        nums
    }
}

#[test]
fn example1() {
    assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
}

#[test]
fn example2() {
    assert_eq!(
        Solution::sequential_digits(1000, 13000),
        vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
    );
}
