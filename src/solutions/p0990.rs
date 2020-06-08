struct UnionFind {
    parent: Box<[usize]>,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            parent: (0..len).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let ret = self.find(self.parent[x]);
            self.parent[x] = ret;
            ret
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        self.parent[self.find(x)] = self.find(y);
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut union_find = UnionFind::new(26);
        let mut list = vec![];
        for equation in equations {
            let bytes = equation.as_bytes();
            let (a, b) = (bytes[0] - b'a', bytes[3] - b'a');
            if bytes[1] == b'=' {
                union_find.union(a as usize, b as usize);
            } else {
                list.push((a, b));
            }
        }
        for (a, b) in list {
            if union_find.find(a as usize) == union_find.find(b as usize) {
                return false;
            }
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_str;

    #[test]
    fn equations_possible() {
        assert!(!Solution::equations_possible(vec_str!["a==b", "b!=a"]));
        assert!(Solution::equations_possible(vec_str![
            "c==c", "b==d", "x!=z"
        ]));
        assert!(!Solution::equations_possible(vec_str![
            "f==a", "a==b", "f!=e", "a==c", "b==e", "c==f"
        ]));
    }
}
