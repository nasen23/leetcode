impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        longest_substring_impl(&s, k)
    }
}

fn longest_substring_impl(s: &str, k: i32) -> i32 {
    let map = s.bytes().fold([0; 26], |mut map, x| {
        map[(x - b'a') as usize] += 1;
        map
    });
    for i in 0..26 {
        if map[i] != 0 && map[i] < k {
            let ch = (b'a' + i as u8) as char;
            return s
                .split(ch)
                .map(|s| longest_substring_impl(s, k))
                .max()
                .unwrap();
        }
    }
    s.len() as i32
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_substring_with_at_least_k_repeating_char() {
        assert_eq!(3, Solution::longest_substring("aaabb".into(), 3));
        assert_eq!(5, Solution::longest_substring("ababbc".into(), 2));
    }
}
