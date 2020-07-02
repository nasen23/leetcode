impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut led = vec![false; 10];
        read_impl(num, &mut led, 0)
    }
}

fn read_impl(n: i32, led: &mut Vec<bool>, start: usize) -> Vec<String> {
    if n == 0 {
        let (h, m) = led
            .iter()
            .enumerate()
            .filter(|(_, &x)| x)
            .map(|(i, _)| i)
            .fold((0, 0), |(h, m), i| {
                if i < 4 {
                    (h + 2usize.pow(i as u32), m)
                } else {
                    (h, m + 2usize.pow(i as u32 - 4))
                }
            });
        if h > 11 || m > 59 {
            return vec![];
        }
        return vec![format!("{}:{:02}", h, m)];
    }
    let mut res = vec![];
    for i in start..led.len() {
        led[i] = true;
        res.append(&mut read_impl(n - 1, led, i + 1));
        led[i] = false;
    }
    res
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{sorted, vec_str};

    #[test]
    fn read_binary_watch() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec_str![
                "1:00", "2:00", "4:00", "8:00", "0:01", "0:02", "0:04", "0:08", "0:16", "0:32"
            ]
        );
        assert_eq!(
            sorted!(Solution::read_binary_watch(2)),
            sorted!(vec_str![
                "0:03", "0:05", "0:06", "0:09", "0:10", "0:12", "0:17", "0:18", "0:20", "0:24",
                "0:33", "0:34", "0:36", "0:40", "0:48", "1:01", "1:02", "1:04", "1:08", "1:16",
                "1:32", "2:01", "2:02", "2:04", "2:08", "2:16", "2:32", "3:00", "4:01", "4:02",
                "4:04", "4:08", "4:16", "4:32", "5:00", "6:00", "8:01", "8:02", "8:04", "8:08",
                "8:16", "8:32", "9:00", "10:00"
            ])
        );
    }
}
