use std::cmp::Ordering;

fn compare(a: &str, b: &str) -> Ordering {
    let (mut ab, mut ba) = (a.to_owned(), b.to_owned());
    ab.push_str(b);
    ba.push_str(a);
    ba.cmp(&ab)
}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| compare(a, b));
        let mut res = nums.join("");
        if res.len() > 1 && res.starts_with('0') {
            res = res.chars().skip_while(|&ch| ch == '0').collect();
            if res.is_empty() {
                res = "0".into();
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn compare_str_special() {
        assert_eq!(compare("3", "34"), Ordering::Greater);
        assert_eq!(compare("34", "3"), Ordering::Less);
        assert_eq!(compare("3", "30"), Ordering::Less);
        assert_eq!(compare("12", "121"), Ordering::Less);
        assert_eq!(compare("8", "883"), Ordering::Less);
    }

    #[test]
    fn largest_number() {
        assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), "9534330");
        assert_eq!(Solution::largest_number(vec![10, 2]), "210");
        assert_eq!(Solution::largest_number(vec![121, 12]), "12121");
        assert_eq!(Solution::largest_number(vec![0, 0, 1, 0, 0]), "10000");
        assert_eq!(Solution::largest_number(vec![0, 0, 0, 0]), "0");
        assert_eq!(
            Solution::largest_number(vec![1440, 7548, 4240, 6616, 733, 4712, 883, 8, 9576]),
            "9576888375487336616471242401440"
        );
        assert_eq!(
            Solution::largest_number(vec![
                4704, 6306, 9385, 7536, 3462, 4798, 5422, 5529, 8070, 6241, 9094, 7846, 663, 6221,
                216, 6758, 8353, 3650, 3836, 8183, 3516, 5909, 6744, 1548, 5712, 2281, 3664, 7100,
                6698, 7321, 4980, 8937, 3163, 5784, 3298, 9890, 1090, 7605, 1380, 1147, 1495, 3699,
                9448, 5208, 9456, 3846, 3567, 6856, 2000, 3575, 7205, 2697, 5972, 7471, 1763, 1143,
                1417, 6038, 2313, 6554, 9026, 8107, 9827, 7982, 9685, 3905, 8939, 1048, 282, 7423,
                6327, 2970, 4453, 5460, 3399, 9533, 914, 3932, 192, 3084, 6806, 273, 4283, 2060,
                5682, 2, 2362, 4812, 7032, 810, 2465, 6511, 213, 2362, 3021, 2745, 3636, 6265,
                1518, 8398
            ]),
            "98909827968595339456944893859149094902689398937839883538183810810780707982784676057536747174237321720571007032685668066758674466986636554651163276306626562416221603859725909578457125682552954605422520849804812479847044453428339323905384638363699366436503636357535673516346233993298316330843021297028227452732697246523622362231322812216213206020001921763154815181495141713801147114310901048"

        );
    }
}
