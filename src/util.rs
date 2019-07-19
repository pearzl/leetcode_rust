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

use std::rc::Rc;
use std::cell::RefCell;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
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
