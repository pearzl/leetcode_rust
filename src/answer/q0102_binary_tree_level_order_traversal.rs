// q0102_binary_tree_level_order_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(rrc_root) = root {
            let mut ret = vec![];
            let mut level_node = vec![rrc_root];
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
                ret.push(level_ret);
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
            Solution::level_order(TreeNode::build(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
        assert_eq!(
            Solution::level_order(TreeNode::build(Vec::<Option<i32>>::new())),
            Vec::<Vec<i32>>::new()
        );
    }
}
