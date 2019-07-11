// q0023_merge_k_sorted_lists

struct Solution;

use crate::util::ListNode;

impl Solution {
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
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(
            ListNode::build(vec![1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                ListNode::build(vec![1, 4, 5]),
                ListNode::build(vec![1, 3, 4]),
                ListNode::build(vec![2, 6])
            ])
        );
    }
}
