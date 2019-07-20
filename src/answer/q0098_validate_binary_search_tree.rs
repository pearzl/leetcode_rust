// q0098_validate_binary_search_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_tree) = root {
            let mut check_list = vec![(i64::min_value(), i64::max_value(), root_tree)];
            while let Some((min, max, node)) = check_list.pop() {
                let cur_val = node.borrow().val as i64;
                if cur_val >= max || cur_val <= min {
                    return false;
                }
                if let Some(ref lt) = node.borrow().left {
                    if (lt.borrow().val as i64) < cur_val {
                        check_list.push((min, cur_val, Rc::clone(lt)));
                    } else {
                        return false;
                    }
                }
                if let Some(ref rt) = node.borrow().right {
                    if (rt.borrow().val as i64) > cur_val {
                        check_list.push((cur_val, max, Rc::clone(rt)));
                    } else {
                        return false;
                    }
                }
            }
            return true;
        } else {
            return true;
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
            Solution::is_valid_bst(TreeNode::build(vec![Some(2), Some(1), Some(3)]))
        );
        assert_eq!(
            false,
            Solution::is_valid_bst(TreeNode::build(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_valid_bst(TreeNode::build(vec![
                Some(10),
                Some(5),
                Some(15),
                None,
                None,
                Some(6),
                Some(20)
            ]))
        );
        assert_eq!(
            false,
            Solution::is_valid_bst(TreeNode::build(vec![
                Some(10),
                Some(5),
                Some(15),
                None,
                None,
                Some(6),
                Some(20)
            ]))
        );
        assert_eq!(
            true,
            Solution::is_valid_bst(TreeNode::build(vec![Some(2147483647)]))
        );
    }
}
