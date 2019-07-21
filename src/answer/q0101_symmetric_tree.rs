// q0101_symmetric_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(t) = root {
            return Solution::symmetric(&t.borrow().left, &t.borrow().right);
        } else {
            return true;
        }
    }

    pub fn symmetric(
        t1: &Option<Rc<RefCell<TreeNode>>>,
        t2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (t1, t2) {
            (None, None) => return true,
            (Some(t1), Some(t2)) => {
                if t1.borrow().val == t2.borrow().val {
                    return Solution::symmetric(&t1.borrow().left, &t2.borrow().right)
                        && Solution::symmetric(&t2.borrow().left, &t1.borrow().right);
                } else {
                    return false;
                }
            }
            _ => return false,
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
            Solution::is_symmetric(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(4),
                Some(4),
                Some(3)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_symmetric(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(2),
                None,
                Some(3),
                None,
                Some(3)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_symmetric(TreeNode::build(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                Some(2)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_symmetric(TreeNode::build(vec![
                Some(5),
                Some(4),
                Some(1),
                None,
                Some(1),
                None,
                Some(4),
                Some(2),
                None,
                Some(2),
                None
            ]))
        );
    }
}
