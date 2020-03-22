struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut most = 0;
        let mut zeros = 1;
        for &i in &flowerbed {
            if i == 0 {
                zeros += 1;
            } else if i == 1 {
                let can_hold = if zeros - 1 < 0 { 0 } else { (zeros - 1) / 2 };
                zeros = 0;
                most += can_hold;
            }
        }
        if zeros >= 2 {
            most += zeros / 2;
        }

        most >= n
    }
}

#[test]
fn case1() {
    assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
}

#[test]
fn case2() {
    assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
}
