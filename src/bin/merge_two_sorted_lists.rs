// 21

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

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let buf = mtl(l1, l2, vec![]);
    let mut ret = None;
    let len = buf.len();
    for i in 1..=len {
        ret = Some(Box::new(ListNode {
            val: buf[len - i],
            next: ret,
        }))
    }
    ret
}

fn mtl(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut v: Vec<i32>) -> Vec<i32> {
    match (l1, l2) {
        (Some(l1), Some(l2)) => {
            if l1.val < l2.val {
                v.push(l1.val);
                return mtl(l1.next, Some(l2), v);
            } else {
                v.push(l2.val);
                return mtl(l2.next, Some(l1), v);
            }
        }
        (Some(l), None) => {
            let mut li = Some(l);
            while let Some(l) = li {
                v.push(l.val);
                if let Some(n) = l.next {
                    li = Some(n)
                } else {
                    break;
                }
            }
            return v;
        }
        (None, Some(l)) => {
            let mut li = Some(l);
            while let Some(l) = li {
                v.push(l.val);
                if let Some(n) = l.next {
                    li = Some(n)
                } else {
                    break;
                }
            }
            return v;
        }
        (None, None) => return v,
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));
    let ret = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        })),
    }));
    assert_eq!(ret, merge_two_lists(l1, l2));
}
