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
}
