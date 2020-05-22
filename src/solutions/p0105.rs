use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        build_tree_impl(&preorder, &inorder)
    }
}

fn build_tree_impl(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }
    let val = preorder[0];
    let left_len = inorder.iter().position(|&x| x == val).unwrap();
    let left = build_tree_impl(&preorder[1..1 + left_len], &inorder[0..left_len]);
    let right = build_tree_impl(&preorder[1 + left_len..], &inorder[1 + left_len..]);
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn build_tree() {
        assert_eq!(
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            None
        );
    }
}
