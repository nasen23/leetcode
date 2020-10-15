impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut f = a[0].chars().fold([0; 26], |mut arr, x| {
            arr[(x as u8 - b'a') as usize] += 1;
            arr
        });
        for s in a.into_iter().skip(1) {
            let tmp = s.chars().fold([0; 26], |mut arr, x| {
                arr[(x as u8 - b'a') as usize] += 1;
                arr
            });
            for i in 0..26 {
                f[i] = f[i].min(tmp[i]);
            }
        }
        let mut res = vec![];
        for (i, &x) in f.iter().enumerate().filter(|(_, &x)| x > 0) {
            for _ in 0..x {
                res.push(String::from((i as u8 + b'a') as char));
            }
        }
        res
    }
}

struct Solution;
