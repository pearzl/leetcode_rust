// q0144_binary_tree_preorder_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(rrct_root) = root {
            let mut child_tree = vec![rrct_root];
            let mut ret = vec![];
            while let Some(rrct) = child_tree.pop() {
                let tree = rrct.borrow();
                ret.push(tree.val);
                if let Some(rt) = tree.right.clone() {
                    child_tree.push(rt);
                }
                if let Some(lt) = tree.left.clone() {
                    child_tree.push(lt);
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
            vec![1, 2, 3],
            Solution::preorder_traversal(TreeNode::build_with_str("[1,null,2,3]"))
        );
    }
}
