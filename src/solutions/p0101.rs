use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(root) => is_symmetric_impl(&root.borrow().left, &root.borrow().right),
            None => true,
        }
    }
}

fn is_symmetric_impl(
    left: &Option<Rc<RefCell<TreeNode>>>,
    right: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => {
            let (left, right) = (left.borrow(), right.borrow());
            if left.val != right.val {
                return false;
            }
            is_symmetric_impl(&left.right, &right.left)
                && is_symmetric_impl(&left.left, &right.right)
        }
        (None, None) => true,
        _ => false,
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::TreeNode;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn is_symmetric() {
        assert!(Solution::is_symmetric(Some(Rc::new(RefCell::new(
            TreeNode::new(10)
        )))));
    }
}
