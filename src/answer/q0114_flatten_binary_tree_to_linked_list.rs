// q0114_flatten_binary_tree_to_linked_list

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Solution::flatten_tree(root.clone());
    }

    fn flatten_tree(
        tree: Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rcc_t) = tree {
            let mut tn = rcc_t.borrow_mut();
            match (&tn.left, &tn.right) {
                (Some(ln), Some(rn)) => {
                    let (nlt, llast_node) = Solution::flatten_tree(Some(Rc::clone(ln)));
                    let (nrt, rlast_node) = Solution::flatten_tree(Some(Rc::clone(rn)));
                    tn.right = nlt;
                    tn.left = None;
                    llast_node.as_ref().unwrap().borrow_mut().right = nrt;
                    return (Some(Rc::clone(&rcc_t)), rlast_node);
                }
                (Some(ln), None) => {
                    let (nlt, last_node) = Solution::flatten_tree(Some(Rc::clone(ln)));
                    tn.right = nlt;
                    tn.left = None;
                    return (Some(Rc::clone(&rcc_t)), last_node);
                }
                (None, Some(rn)) => {
                    let (_, last_node) = Solution::flatten_tree(Some(Rc::clone(rn)));
                    return (Some(Rc::clone(&rcc_t)), last_node);
                }
                (None, None) => return (Some(Rc::clone(&rcc_t)), Some(Rc::clone(&rcc_t))),
            }
        } else {
            return (None, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        let mut input = TreeNode::build_with_str("[1,2,5,3,4,null,6]");
        Solution::flatten(&mut input);
        assert_eq!(
            TreeNode::build_with_str("[1,null,2,null,3,null,4,null,5,null,6]"),
            input
        );

        let mut input = TreeNode::build_with_str("[1,2,3]");
        Solution::flatten(&mut input);
        assert_eq!(TreeNode::build_with_str("[1,null,2,null,3]"), input);
    }
}
