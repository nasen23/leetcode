use crate::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(0, 1);
        prefix_iter(&root, sum, &mut map, 0)
    }
}

fn prefix_iter(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sum: i32,
    map: &mut HashMap<i32, i32>,
    mut prefix: i32,
) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            prefix += node.val;
            let mut res = map.get(&(prefix - sum)).cloned().unwrap_or(0);
            let count = map.entry(prefix).or_insert(0);
            *count += 1;
            res += prefix_iter(&node.left, sum, map, prefix)
                + prefix_iter(&node.right, sum, map, prefix);
            map.get_mut(&prefix).map(|x| *x -= 1);
            res
        }
        None => 0,
    }
}

struct Solution;
