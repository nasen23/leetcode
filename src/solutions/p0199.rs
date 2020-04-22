use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn right_side_view(root: Tree) -> Vec<i32> {
        let mut depth = 0;
        let mut queue = VecDeque::new();
        let mut res = vec![];
        queue.push_back((root, depth));
        while let Some((node, d)) = queue.pop_front() {
            if let Some(node) = node {
                let node = Rc::try_unwrap(node).unwrap().into_inner();
                if depth == d {
                    res.push(node.val);
                    depth += 1;
                }
                queue.push_back((node.right, depth));
                queue.push_back((node.left, depth));
            }
        }

        res
    }
}

struct Solution;
