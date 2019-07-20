#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for i in 1..=v.len() {
            ret = Some(Box::new(ListNode {
                val: v[v.len() - i],
                next: ret,
            }))
        }
        ret
    }

    pub fn into_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut tmp = vec![];
        let mut head = head;
        while let Some(ln) = head {
            tmp.push(ln.val);
            head = ln.next;
        }
        tmp
    }
}

use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::collections::VecDeque;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if v.len() == 0 {
            return None;
        }
        let mut v_iter = v.into_iter();
        let head_v = v_iter.next().unwrap().unwrap();
        let head = Rc::new(RefCell::new(TreeNode::new(head_v)));
        let mut tmp = VecDeque::new();
        tmp.push_back(Rc::clone(&head));
        while let Some(node) = tmp.pop_front() {
            if let Some(lv) = v_iter.next() {
                match lv {
                    Some(val) => {
                        let ln = Rc::new(RefCell::new(TreeNode::new(val)));
                        tmp.push_back(Rc::clone(&ln));
                        node.borrow_mut().left = Some(ln);
                    }
                    None => {
                        node.borrow_mut().left = None;
                    }
                }
            } else {
                break;
            }
            if let Some(rv) = v_iter.next() {
                match rv {
                    Some(val) => {
                        let rn = Rc::new(RefCell::new(TreeNode::new(val)));
                        tmp.push_back(Rc::clone(&rn));
                        node.borrow_mut().right = Some(rn);
                    }
                    None => {
                        node.borrow_mut().right = None;
                    }
                }
            } else {
                break;
            }
        }
        Some(head)
    }

    pub fn clone_tree(t: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match t {
            None => return None,
            Some(rf_node) => {
                return Some(Rc::new(RefCell::new(TreeNode {
                    val: rf_node.borrow().val,
                    left: TreeNode::clone_tree(&rf_node.borrow().left),
                    right: TreeNode::clone_tree(&rf_node.borrow().right),
                })))
            }
        }
    }

    pub fn into_vec(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut queue = VecDeque::new();
        queue.push_back(tree);
        let mut ret = vec![];
        while let Some(node) = queue.pop_front() {
            match node {
                None => {
                    ret.push(None);
                }
                Some(t) => {
                    ret.push(Some(t.borrow().val));
                    if let Some(ref lt) = t.borrow().left {
                        queue.push_back(Some(Rc::clone(lt)));
                    } else {
                        queue.push_back(None);
                    }
                    if let Some(ref rt) = t.borrow().right {
                        queue.push_back(Some(Rc::clone(rt)));
                    } else {
                        queue.push_back(None);
                    }
                }
            }
        }
        while let Some(item) = ret.pop() {
            if let Some(t) = item {
                ret.push(Some(t));
                break;
            }
        }
        ret
    }
}

use std::collections::HashSet;

pub fn vec_2_set<T: Clone + std::hash::Hash + Eq>(v: Vec<T>) -> HashSet<T> {
    v.iter().cloned().collect()
}

pub fn compare_nest2_vec<T: Ord + Clone + std::hash::Hash + Eq>(
    a: Vec<Vec<T>>,
    b: Vec<Vec<T>>,
) -> bool {
    let aa: HashSet<Vec<T>> = a
        .into_iter()
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .collect();
    let bb: HashSet<Vec<T>> = b
        .into_iter()
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .collect();
    aa == bb
}

pub fn build_sudo(s: [[&str; 9]; 9]) -> Vec<Vec<char>> {
    let mut ret = vec![];
    for line in s.iter() {
        ret.push(line.iter().map(|c| c.chars().next().unwrap()).collect());
    }
    ret
}

pub fn print_sudo<T: std::fmt::Debug>(sudo: Vec<Vec<T>>) {
    for v in sudo.iter() {
        println!("{:?}", v);
    }
}

#[cfg(test)]
mod test {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn build_tree_node() {
        assert_eq!(
            TreeNode::build(vec![Some(1), None, Some(2), Some(3)]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))
        );
    }

    #[test]
    fn clone_tree() {
        let t1 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let t2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: TreeNode::clone_tree(&t1),
            right: None,
        })));
        t1.as_ref().unwrap().borrow_mut().val = 10;
        assert_eq!(TreeNode::build(vec![Some(2), Some(1)]), t2);
        assert_eq!(TreeNode::build(vec![Some(10)]), t1);
    }

    #[test]
    fn tree_to_vec() {
        let v1 = vec![Some(1), None, Some(2)];
        let t1 = TreeNode::build(v1.clone());
        assert_eq!(v1, TreeNode::into_vec(t1));
        let v2 = vec![Some(2), Some(1)];
        let t2 = TreeNode::build(v2.clone());
        assert_eq!(v2, TreeNode::into_vec(t2));
    }
}
