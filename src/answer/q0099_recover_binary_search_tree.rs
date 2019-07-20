// q0099_recover_binary_search_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rrct_root) = root {
            let mut traver_buf: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(rrct_root)];
            let mut compare_node: Option<Rc<RefCell<TreeNode>>> = None;
            let mut exchange_node: Option<Rc<RefCell<TreeNode>>> = None;
            let mut exchange_node2: Option<Rc<RefCell<TreeNode>>> = None;
            let mut travered_node: Vec<Rc<RefCell<TreeNode>>> = vec![];
            while let Some(node) = traver_buf.pop() {
                match (&node.borrow().left, &node.borrow().right) {
                    (Some(ref ln), Some(ref rn)) => {
                        if travered_node.contains(&node) {
                            if let Some(t) = compare_node {
                                if t.borrow().val >= node.borrow().val {
                                    if exchange_node.is_none() {
                                        exchange_node = Some(Rc::clone(&t));
                                        exchange_node2 = Some(Rc::clone(&node));
                                    } else {
                                        exchange_node2 = Some(Rc::clone(&node));
                                        break;
                                    }
                                }
                            }
                            compare_node = Some(Rc::clone(&node));
                        } else {
                            traver_buf.push(Rc::clone(rn));
                            // traver_buf.push(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
                            traver_buf.push(Rc::clone(&node));
                            travered_node.push(Rc::clone(&node));
                            traver_buf.push(Rc::clone(ln));
                        }
                    }
                    (Some(ref ln), None) => {
                        if travered_node.contains(&node) {
                            if let Some(t) = compare_node {
                                if t.borrow().val >= node.borrow().val {
                                    if exchange_node.is_none() {
                                        exchange_node = Some(Rc::clone(&t));
                                        exchange_node2 = Some(Rc::clone(&node));
                                    } else {
                                        exchange_node2 = Some(Rc::clone(&node));
                                        break;
                                    }
                                }
                            }
                            compare_node = Some(Rc::clone(&node));
                        } else {
                            // traver_buf.push(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))));
                            traver_buf.push(Rc::clone(&node));
                            travered_node.push(Rc::clone(&node));
                            traver_buf.push(Rc::clone(ln));
                        }
                    }
                    (None, Some(ref rn)) => {
                        traver_buf.push(Rc::clone(rn));
                        if let Some(t) = compare_node {
                            if t.borrow().val >= node.borrow().val {
                                if exchange_node.is_none() {
                                    exchange_node = Some(Rc::clone(&t));
                                    exchange_node2 = Some(Rc::clone(&node));
                                } else {
                                    exchange_node2 = Some(Rc::clone(&node));
                                    break;
                                }
                            }
                        }
                        compare_node = Some(Rc::clone(&node));
                    }
                    (None, None) => {
                        if let Some(t) = compare_node {
                            if t.borrow().val >= node.borrow().val {
                                if exchange_node.is_none() {
                                    exchange_node = Some(Rc::clone(&t));
                                    exchange_node2 = Some(Rc::clone(&node));
                                } else {
                                    exchange_node2 = Some(Rc::clone(&node));
                                    break;
                                }
                            }
                        }
                        compare_node = Some(Rc::clone(&node));
                    }
                }
            }
            match (exchange_node, exchange_node2) {
                (Some(n1), Some(n2)) => {
                    let t = n1.borrow().val;
                    n1.borrow_mut().val = n2.borrow().val;
                    n2.borrow_mut().val = t;
                }
                _ => panic!("unexpected"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        let mut r = TreeNode::build(vec![Some(1), Some(3), None, None, Some(2)]);
        Solution::recover_tree(&mut r);
        assert_eq!(
            TreeNode::build(vec![Some(3), Some(1), None, None, Some(2)]),
            r
        );

        let mut r = TreeNode::build(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
        Solution::recover_tree(&mut r);
        assert_eq!(
            TreeNode::build(vec![Some(2), Some(1), Some(4), None, None, Some(3)]),
            r
        );
    }
}
