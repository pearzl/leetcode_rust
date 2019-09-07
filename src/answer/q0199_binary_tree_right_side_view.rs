// q0199_binary_tree_right_side_view

struct Solution;
use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ret = vec![];
        let mut node_buf = vec![root.unwrap()];
        while !node_buf.is_empty() {
            let mut next_level_nodes = Vec::with_capacity(node_buf.len() * 2);
            for n in node_buf.iter() {
                let t = n.borrow();
                if let Some(ref lt) = t.left {
                    next_level_nodes.push(Rc::clone(lt));
                }
                if let Some(ref rt) = t.right {
                    next_level_nodes.push(Rc::clone(rt));
                }
            }
            let last_node = node_buf.pop().unwrap();
            std::mem::replace(&mut node_buf, next_level_nodes);
            let t = last_node.borrow();
            ret.push(t.val);
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(TreeNode::build_with_str("[1,2,3,null,5,null,4]"))
        );
        assert_eq!(
            vec![1, 3, 4],
            Solution::right_side_view(TreeNode::build_with_str("[1,2,3,4]"))
        );
    }
}
