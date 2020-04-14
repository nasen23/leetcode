use crate::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_bottom_left_value(root: Tree) -> i32 {
        let mut queue = VecDeque::with_capacity(16);

        queue.push_front(root);
        let mut node = TreeNode::new(0);
        while !queue.is_empty() {
            node = Rc::try_unwrap(queue.pop_back().unwrap().unwrap())
                .unwrap()
                .into_inner();
            if node.right.is_some() {
                queue.push_front(node.right);
            }
            if node.left.is_some() {
                queue.push_front(node.left);
            }
        }

        node.val
    }
}

struct Solution;
