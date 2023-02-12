impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut res = String::new();
        let mut pos = (0, 0);
        for ch in target.into_bytes() {
            let num = ch - b'a';
            let move_to = ((num / 5) as i8, (num % 5) as i8);
            let (y, x) = (move_to.0 - pos.0, move_to.1 - pos.1);
            let xop = if x < 0 { 'L' } else { 'R' };
            let yop = if y < 0 { 'U' } else { 'D' };
            if pos == (5, 0) {
                for _ in 0..y.abs() {
                    res.push(yop);
                }
                for _ in 0..x.abs() {
                    res.push(xop);
                }
            } else {
                for _ in 0..x.abs() {
                    res.push(xop);
                }
                for _ in 0..y.abs() {
                    res.push(yop);
                }
            }
            res.push('!');
            pos = move_to;
        }
        res
    }
}

struct Solution;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_board_path() {
        assert_eq!(Solution::alphabet_board_path("leet".into()), "DDR!UURRR!!DDD!");
    }
}
