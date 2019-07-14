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
}

use std::collections::HashSet;

pub fn vec_2_set<T: Clone + std::hash::Hash + Eq>(v: Vec<T>) -> HashSet<T> {
    v.iter().cloned().collect()
}

pub fn compare_nest2_vec<T: Ord + Clone + std::hash::Hash + Eq>(
    mut a: Vec<Vec<T>>,
    mut b: Vec<Vec<T>>,
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

pub fn print_sudo(sudo: Vec<Vec<char>>) {
    for v in sudo.iter() {
        println!("{:?}", v);
    }
}
