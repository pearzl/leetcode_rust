// q0103_binary_tree_zigzag_level_order_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(rrc_root) = root {
            let mut ret = vec![];
            let mut level_node = vec![rrc_root];
            let mut reserve = false;
            loop {
                let mut new_level_node = vec![];
                let mut level_ret = vec![];
                for node in level_node.iter() {
                    let val = node.borrow().val;
                    level_ret.push(val);
                    if let Some(ref ln) = node.borrow().left {
                        new_level_node.push(Rc::clone(ln));
                    }
                    if let Some(ref rn) = node.borrow().right {
                        new_level_node.push(Rc::clone(rn));
                    }
                }
                if !reserve {
                    ret.push(level_ret);
                } else {
                    let reserved = level_ret.into_iter().rev().collect();
                    ret.push(reserved);
                }
                reserve = !reserve;
                if new_level_node.is_empty() {
                    break;
                } else {
                    std::mem::replace(&mut level_node, new_level_node);
                }
            }
            return ret;
        } else {
            return vec![];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::zigzag_level_order(TreeNode::build(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
        assert_eq!(
            Solution::zigzag_level_order(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(5)
            ])),
            vec![vec![1], vec![3, 2], vec![4, 5]]
        );
    }
}
