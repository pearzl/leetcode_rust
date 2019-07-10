// 23

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

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut bufv: Vec<Vec<i32>> = vec![vec![]; lists.len()];
    for (i, mut ol) in lists.into_iter().enumerate() {
        while let Some(list) = ol {
            bufv[i].push(list.val);
            ol = list.next;
        }
    }
    let mut tmp = vec![];
    for mut i in bufv.into_iter() {
        tmp.append(&mut i);
    }
    tmp.sort_unstable();

    let len = tmp.len();
    let mut ret = None;
    for i in 1..=len {
        ret = Some(Box::new(ListNode {
            val: tmp[len - i],
            next: ret,
        }))
    }
    ret
}

fn main() {
    assert_eq!(
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        })),
        merge_k_lists(vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 42,
                next: Some(Box::new(ListNode { val: 6, next: None }))
            }))
        ])
    )
}
