use std::collections::{HashMap, HashSet};

struct Twitter {
    tweets: Vec<(i32, i32)>,
    followees: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            tweets: Vec::with_capacity(8),
            followees: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.tweets.push((user_id, tweet_id));
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.tweets
            .iter()
            .rev()
            .filter(|(x, _)| {
                *x == user_id
                    || self
                        .followees
                        .get(&user_id)
                        .map_or(false, |s| s.contains(x))
            })
            .map(|(_, x)| *x)
            .take(10)
            .collect()
    }

    fn follow(&mut self, follower: i32, followee: i32) {
        match self.followees.get_mut(&follower) {
            Some(s) => {
                s.insert(followee);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(followee);
                self.followees.insert(follower, set);
            }
        }
    }

    fn unfollow(&mut self, follower: i32, followee: i32) {
        match self.followees.get_mut(&follower) {
            Some(s) => {
                s.remove(&followee);
            }
            None => {
                self.followees.insert(follower, HashSet::new());
            }
        }
    }
}
