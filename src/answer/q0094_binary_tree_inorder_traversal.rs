// q0094_binary_tree_inorder_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(head) => {
                let mut ret = vec![];
                let mut tree_buf = vec![head];
                while let Some(rf_node) = tree_buf.pop() {
                    match (&rf_node.borrow().left.as_ref(), &rf_node.borrow().right) {
                        (Some(ref ln), Some(ref rn)) => {
                            tree_buf.push(Rc::clone(rn));
                            tree_buf
                                .push(Rc::new(RefCell::new(TreeNode::new(rf_node.borrow().val))));
                            tree_buf.push(Rc::clone(ln));
                        }
                        (Some(ref ln), None) => {
                            tree_buf
                                .push(Rc::new(RefCell::new(TreeNode::new(rf_node.borrow().val))));
                            tree_buf.push(Rc::clone(ln));
                        }
                        (None, Some(ref rn)) => {
                            tree_buf.push(Rc::clone(rn));
                            ret.push(rf_node.borrow().val);
                        }
                        (None, None) => {
                            ret.push(rf_node.borrow().val);
                        }
                    }
                }
                return ret;
            }
            None => return vec![],
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
            vec![1, 3, 2],
            Solution::inorder_traversal(TreeNode::build(vec![Some(1), None, Some(2), Some(3)]))
        );
    }
}
