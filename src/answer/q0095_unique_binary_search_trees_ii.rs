// q0095_unique_binary_search_trees_ii

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }
        let mut buf: Vec<Vec<Option<Rc<RefCell<TreeNode>>>>> =
            (0..n).into_iter().map(|_| vec![]).collect();
        buf[0].push(Some(Rc::new(RefCell::new(TreeNode::new(1)))));
        Solution::gt(&mut buf, n as usize);
        buf.pop().unwrap()
    }

    pub fn gt(
        buf: &mut Vec<Vec<Option<Rc<RefCell<TreeNode>>>>>,
        n: usize,
    ) -> &Vec<Option<Rc<RefCell<TreeNode>>>> {
        if buf[n - 1].len() != 0 {
            return &buf[n - 1];
        }
        for i in 0..Solution::gt(buf, n - 1).len() {
            let nt1 = Some(Rc::new(RefCell::new(TreeNode {
                val: n as i32,
                left: Solution::clone_tree(&buf[n - 2][i]),
                right: None,
            })));
            buf[n - 1].push(nt1);
            let nt2 = Solution::clone_tree(&buf[n - 2][i]).unwrap();
            let new_trees = Solution::insert_right(nt2, n as i32);
            for t in new_trees.into_iter() {
                buf[n - 1].push(Some(t));
            }
        }
        return &buf[n - 1];
    }

    pub fn insert_right(tree: Rc<RefCell<TreeNode>>, ele: i32) -> Vec<Rc<RefCell<TreeNode>>> {
        let tr = tree.borrow();
        match tr.right {
            None => {
                let new_tree = Solution::clone_tree(&Some(Rc::clone(&tree))).unwrap();
                new_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(ele))));
                vec![new_tree]
            }
            Some(ref rft) => {
                let mut ret = vec![];
                let t = Solution::insert_right(
                    Solution::clone_tree(&Some(Rc::clone(rft))).unwrap(),
                    ele,
                );
                for ttr in t.into_iter() {
                    let nt = Solution::clone_tree(&Some(Rc::clone(&tree))).unwrap();
                    nt.borrow_mut().right = Some(ttr);
                    ret.push(nt);
                }
                let new_tree = Solution::clone_tree(&Some(Rc::clone(&tree))).unwrap();
                new_tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                    val: ele,
                    left: Some(Rc::clone(rft)),
                    right: None,
                })));
                ret.push(new_tree);
                ret
            }
        }
    }

    pub fn clone_tree(t: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match t {
            None => return None,
            Some(rf_node) => {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: rf_node.borrow().val,
                    left: Solution::clone_tree(&rf_node.borrow().left),
                    right: Solution::clone_tree(&rf_node.borrow().right),
                })))
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
        assert_eq!(
            vec![TreeNode::build(vec![Some(1)])],
            Solution::generate_trees(1)
        );
        assert_eq!(
            vec![
                TreeNode::build(vec![Some(2), Some(1)]),
                TreeNode::build(vec![Some(1), None, Some(2)])
            ],
            Solution::generate_trees(2)
        );
        assert_eq!(
            vec![
                TreeNode::build(vec![Some(3), Some(2), None, Some(1)]),
                TreeNode::build(vec![Some(2), Some(1), Some(3)]),
                TreeNode::build(vec![Some(3), Some(1), None, None, Some(2)]),
                TreeNode::build(vec![Some(1), None, Some(2), None, Some(3)]),
                TreeNode::build(vec![Some(1), None, Some(3), Some(2)]),
            ],
            Solution::generate_trees(3)
        );
    }
}
