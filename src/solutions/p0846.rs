use std::collections::HashMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, w: i32) -> bool {
        if hand.len() % w as usize != 0 {
            return false;
        }
        if w == 1 {
            return true;
        }
        let mut hand = hand;
        hand.sort_unstable();
        let mut map = hand.iter().fold(HashMap::new(), |mut map, &x| {
            let c = map.entry(x).or_insert(0);
            *c += 1;
            map
        });
        for h in hand {
            if let Some(c) = map.get_mut(&h) {
                if *c == 0 {
                    continue;
                }
                *c -= 1;
                for j in 1..w {
                    if let Some(c) = map.get_mut(&(h + j)) {
                        if *c > 0 {
                            *c -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_of_straights() {
        assert_eq!(Solution::is_n_straight_hand(vec![5, 1], 2), false);
    }
}
