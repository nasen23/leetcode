use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mask = (1 << n) - 1;
        if n == 0 {
            return vec![];
        }
        gen_trees_impl(mask, 0)
    }
}

fn gen_trees_impl(mask: i32, i: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if i & mask == mask {
        return vec![None];
    }
    let mut res = vec![];
    let mut bitmap = mask & !i;
    while bitmap > 0 {
        let pick = bitmap & -bitmap;
        let x = pick.trailing_zeros() as i32 + 1;
        let l = gen_trees_impl(mask, i | !(pick - 1));
        let r = gen_trees_impl(mask, i | ((pick - 1) << 1) + 1);
        for l in &l {
            for r in &r {
                res.push(Some(Rc::new(RefCell::new(TreeNode {
                    val: x,
                    left: l.clone(),
                    right: r.clone(),
                }))))
            }
        }
        bitmap ^= pick;
    }
    res
}

use crate::TreeNode;
struct Solution;
