use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back((root.unwrap(), 1));
        while let Some((node, d)) = queue.pop_front() {
            let node = Rc::try_unwrap(node).unwrap().into_inner();
            if node.left.is_none() && node.right.is_none() {
                return d;
            }
            if let Some(left) = node.left {
                queue.push_back((left, d + 1));
            }
            if let Some(right) = node.right {
                queue.push_back((right, d + 1));
            }
        }
        unreachable!()
    }
}

struct Solution;
use crate::TreeNode;
