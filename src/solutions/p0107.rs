use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back((root, 0));
        let mut res = vec![];
        while let Some((node, d)) = queue.pop_front() {
            if let Some(node) = node {
                let node = Rc::try_unwrap(node).unwrap().into_inner();
                if d == res.len() {
                    res.push(vec![node.val]);
                } else {
                    res[d].push(node.val);
                }
                queue.push_back((node.left, d + 1));
                queue.push_back((node.right, d + 1));
            }
        }
        res.reverse();
        res
    }
}

struct Solution;
use crate::TreeNode;
