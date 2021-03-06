// q0090_subsets_ii

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let n = nums.len() as i32;
        for i in 0..=n {
            let mut t = Solution::combine(n, i);
            ret.append(&mut t);
        }
        let mut rmdup = vec![];
        for r in ret.iter_mut() {
            *r = r.iter().map(|x| nums[*x as usize - 1]).collect();
            (*r).sort_unstable();
            if !rmdup.contains(r) {
                let t = std::mem::replace(r, vec![]);
                rmdup.push(t);
            }
        }
        rmdup
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return vec![vec![]];
        }
        if n < k {
            return vec![];
        }
        let mut ret = vec![];
        for rb in k..=n {
            let mut init: Vec<i32> = (1..k).into_iter().collect();
            init.push(rb);
            let mut tmpr = vec![];
            let mut i = init.len() - 1;
            tmpr.push(init);
            while i > 0 {
                for index in 0..tmpr.len() {
                    let item = tmpr[index].clone();
                    let mut j = 1;
                    while item[i] != item[i - 1] + j {
                        let mut t = item.clone();
                        t[i - 1] += j;
                        tmpr.push(t);
                        j += 1;
                    }
                }
                i -= 1;
            }
            ret.append(&mut tmpr);
            tmpr.clear();
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(vec![
                vec![2],
                vec![1],
                vec![1, 2, 2],
                vec![2, 2],
                vec![1, 2],
                vec![]
            ]),
            util::vec_2_set(Solution::subsets_with_dup(vec![1, 2, 2]))
        );
    }
}
