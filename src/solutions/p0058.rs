impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.bytes()
            .rev()
            .skip_while(|&c| c == b' ')
            .take_while(|&c| c != b' ')
            .count() as i32
    }

    pub fn length_of_last_word_chars(s: String) -> i32 {
        s.chars()
            .rev()
            .skip_while(|&b| b == ' ')
            .take_while(|&b| b != ' ')
            .count() as i32
    }

    pub fn length_of_last_word_loop(s: String) -> i32 {
        let mut s = s;
        let mut res = 0;

        while let Some(c) = s.pop() {
            if c == ' ' && res == 0 {
                continue;
            } else if c == ' ' && res != 0 {
                break;
            } else {
                res += 1;
            }
        }

        res
    }

    pub fn length_of_last_word_split(s: String) -> i32 {
        let ve: Vec<&str> = s.trim().rsplit(' ').collect();
        if ve.is_empty() {
            0
        } else {
            ve[0].len() as i32
        }
    }
}

struct Solution;

static HELLO_WORLD_STR: &str = "Hello                        world                             ";

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::{Solution, HELLO_WORLD_STR};

    #[bench]
    fn using_chars(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word_chars(HELLO_WORLD_STR.to_string()))
    }

    #[bench]
    fn using_bytes(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word(HELLO_WORLD_STR.to_string()))
    }

    #[bench]
    fn using_loop(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word_loop(HELLO_WORLD_STR.to_string()))
    }

    #[bench]
    fn using_split(b: &mut Bencher) {
        b.iter(|| Solution::length_of_last_word_split(HELLO_WORLD_STR.to_string()))
    }
}
