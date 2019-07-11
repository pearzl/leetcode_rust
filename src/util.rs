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

pub fn vec_2_set<T: Clone + std::hash::Hash + Eq>(v: Vec<T>) -> std::collections::HashSet<T> {
    v.iter().cloned().collect()
}
