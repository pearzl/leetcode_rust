// q0047_permutations_ii

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for (i, v) in nums.iter().cloned().enumerate() {
            let tmp = vec![v];
            let netp: Vec<i32> = nums
                .iter()
                .cloned()
                .enumerate()
                .filter(|(j, n)| *j != i)
                .map(|x| x.1)
                .collect();
            let mut tmp2 = Solution::permute_unique(netp);
            if tmp2.len() == 0 {
                ret.push(tmp);
            } else {
                for v in tmp2.iter_mut() {
                    let mut t = tmp.clone();
                    t.append(v);
                    if !ret.contains(&t) {
                        ret.push(t);
                    }
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }
}
