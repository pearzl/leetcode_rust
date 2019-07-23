// q0110_balanced_binary_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rrc_t) = root {
            let tree = rrc_t.borrow();
            let ldep = Solution::max_depth(tree.left.clone());
            let rdep = Solution::max_depth(tree.right.clone());
            if (ldep - rdep).abs() > 1 {
                return false;
            }
            return Solution::is_balanced(tree.left.clone())
                && Solution::is_balanced(tree.right.clone());
        } else {
            return true;
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rrc) = root {
            match (&rrc.borrow().left, &rrc.borrow().right) {
                (Some(ln), Some(rn)) => {
                    let d1 = Solution::max_depth(Some(Rc::clone(ln)));
                    let d2 = Solution::max_depth(Some(Rc::clone(rn)));
                    return 1 + d1.max(d2);
                }
                (Some(ln), None) => return 1 + Solution::max_depth(Some(Rc::clone(ln))),
                (None, Some(rn)) => return 1 + Solution::max_depth(Some(Rc::clone(rn))),
                (None, None) => return 1,
            }
        } else {
            return 0;
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
            true,
            Solution::is_balanced(TreeNode::build(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_balanced(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3),
                None,
                None,
                Some(4),
                Some(4)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_balanced(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                None,
                None,
                Some(3),
                Some(4),
                None,
                None,
                Some(4)
            ]))
        );
    }
}
