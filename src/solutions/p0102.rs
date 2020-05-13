use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));
        let mut res: Vec<Vec<i32>> = vec![];
        while let Some((node, depth)) = queue.pop_front() {
            match node {
                Some(node) => {
                    let node = Rc::try_unwrap(node).unwrap().into_inner();
                    match res.get_mut(depth) {
                        Some(ve) => ve.push(node.val),
                        None => {
                            let ve = vec![node.val];
                            res.push(ve);
                        }
                    }
                    queue.push_back((node.left, depth + 1));
                    queue.push_back((node.right, depth + 1));
                }
                None => continue,
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::{vec_vec, TreeNode};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn level_order() {
        assert_eq!(
            Solution::level_order(Some(Rc::new(RefCell::new(TreeNode::new(10))))),
            vec_vec![[10]]
        );
    }
}
