// 24

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

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = head;
    let mut buf = vec![];
    while let Some(l) = list {
        buf.push(l.val);
        list = l.next;
    }
    if buf.len() < 1 {
        return None;
    }
    let mut i = 0;
    while i < buf.len() - 1 {
        let t = buf[i];
        buf[i] = buf[i + 1];
        buf[i + 1] = t;
        i += 2;
    }

    let len = buf.len();
    let mut ret = None;
    for i in 1..=len {
        ret = Some(Box::new(ListNode {
            val: buf[len - i],
            next: ret,
        }))
    }
    ret
}

fn main() {
    assert_eq!(
        swap_pairs(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        }))
    )
}
