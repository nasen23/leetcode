use std::cmp;
use std::collections::BTreeMap;

struct RangeModule {
    ranges: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        let (mut left, mut right) = (left, right);
        let rl: Vec<_> = self
            .ranges
            .range(left..)
            .take_while(|(_, &l)| l <= right)
            .map(|(&r, &l)| (r, l))
            .collect();
        for (r, l) in rl {
            left = cmp::min(left, l);
            right = cmp::max(right, r);
            self.ranges.remove(&r);
        }

        self.ranges.insert(right, left);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self.ranges.range(right..).any(|(_, &l)| l <= left)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        let rl: Vec<_> = self
            .ranges
            .range(left..)
            .take_while(|(_, &l)| l < right)
            .map(|(&r, &l)| (r, l))
            .collect();
        for (r, l) in rl {
            self.ranges.remove(&r);
            if left > l {
                self.ranges.insert(left, l);
            }
            if right < r {
                self.ranges.insert(r, right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RangeModule;

    #[test]
    fn range_module() {
        let mut module = RangeModule::new();
        module.add_range(10, 20);
        assert!(module.query_range(15, 16));
        module.remove_range(12, 16);
        assert!(!module.query_range(15, 16));
        assert!(module.query_range(10, 11));
    }
}
