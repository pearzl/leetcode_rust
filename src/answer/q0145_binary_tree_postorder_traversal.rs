// q0145_binary_tree_postorder_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(rrct_root) = root {
            let mut child_tree = vec![rrct_root];
            let mut ret = vec![];
            while let Some(rrct_tree) = child_tree.pop() {
                let mut tree = rrct_tree.borrow_mut();
                match (tree.left.clone(), tree.right.clone()) {
                    (Some(lt), Some(rt)) => {
                        std::mem::replace(&mut tree.left, None);
                        std::mem::replace(&mut tree.right, None);
                        child_tree.push(Rc::clone(&rrct_tree));
                        child_tree.push(rt);
                        child_tree.push(lt);
                    }
                    (Some(lt), None) => {
                        std::mem::replace(&mut tree.left, None);
                        child_tree.push(Rc::clone(&rrct_tree));
                        child_tree.push(lt);
                    }
                    (None, Some(rt)) => {
                        std::mem::replace(&mut tree.right, None);
                        child_tree.push(Rc::clone(&rrct_tree));
                        child_tree.push(rt);
                    }
                    (None, None) => {
                        ret.push(tree.val);
                    }
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
            vec![3, 2, 1],
            Solution::postorder_traversal(TreeNode::build_with_str("[1,null,2,3]"))
        );
    }
}
