// q0173_binary_search_tree_iterator

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
struct BSTIterator {
    buf: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let buf = if let Some(b) = root { vec![b] } else { vec![] };
        BSTIterator { buf }
    }

    /** @return the next smallest number */
    fn next(&mut self) -> i32 {
        let rrct = self.buf.pop().unwrap();
        let mut tn = rrct.borrow_mut();
        if let Some(lt) = tn.left.clone() {
            tn.left = None;
            self.buf.push(Rc::clone(&rrct));
            self.buf.push(Rc::clone(&lt));
            return self.next();
        }
        if let Some(ref rt) = tn.right {
            self.buf.push(Rc::clone(rt));
        }
        return tn.val;
    }

    /** @return whether we have a next smallest number */
    fn has_next(&self) -> bool {
        !self.buf.is_empty()
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    // use super::Solution;

    use super::BSTIterator;

    use crate::util;
    use crate::util::TreeNode;

    use std::cell::RefCell;
    use std::rc::Rc;

    fn run(actions: Vec<&str>, tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut bstt = BSTIterator::new(tree);
        let mut ret = vec![];
        for act in actions.into_iter() {
            if act == "BSTIterator" {
                // bstt = BSTIterator::new(tree);
                ret.push(String::from("null"));
            } else if act == "next" {
                let r = bstt.next();
                ret.push(format!("{}", r));
            } else if act == "hasNext" {
                let r = bstt.has_next();
                ret.push(format!("{}", r));
            } else {
                panic!(1234567890);
            }
        }
        ret
    }

    #[test]
    fn it_works() {
        assert_eq!(
            util::build_string_array(vec![
                "null", "3", "7", "true", "9", "true", "15", "true", "20", "false"
            ]),
            run(
                vec![
                    "BSTIterator",
                    "next",
                    "next",
                    "hasNext",
                    "next",
                    "hasNext",
                    "next",
                    "hasNext",
                    "next",
                    "hasNext"
                ],
                TreeNode::build_with_str("[7,3,15,null,null,9,20]")
            )
        );
    }
}
