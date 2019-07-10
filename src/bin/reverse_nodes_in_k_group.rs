// 25

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
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut tmp = vec![];
    let mut buf = vec![];
    let mut count = 0;
    let mut head = head;
    while let Some(l) = head {
        tmp.push(l.val);
        head = l.next;
        count += 1;
        if count == k {
            buf.push(tmp.clone());
            tmp.clear();
            count = 0;
        }
    }
    if tmp.len() != 0 {
        buf.push(tmp);
    }
    let mut ret = vec![];
    for i in buf.iter_mut() {
        if i.len() != k as usize {
            ret.append(&mut *i)
        }else{
            while let Some(item) = i.pop() {
                ret.push(item);
            }
        }
    }
    build(ret)
}

fn main() {
    assert_eq!(
        reverse_k_group(build(vec![1, 2, 3, 4, 5]), 2),
        build(vec![2, 1, 4, 3, 5])
    );
    assert_eq!(
        reverse_k_group(build(vec![1, 2, 3, 4, 5]), 3),
        build(vec![3, 2, 1, 4, 60])
    );
    // println!("{:?}", build(vec![1,2,3,4,5,5]));
}

fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut ret = None;
    for i in 1..=v.len() {
        ret = Some(Box::new(ListNode {
            val: v[v.len() - i],
            next: ret,
        }))
    }
    ret
}
