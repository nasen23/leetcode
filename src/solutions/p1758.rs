impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let t = s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    if c == '1' {
                        '0'
                    } else {
                        '1'
                    }
                } else {
                    c
                }
            })
            .collect::<String>();
        let res = t.chars().filter(|&ch| ch == '0').count();
        res.min(t.len() - res) as i32
    }
}

struct Solution;
