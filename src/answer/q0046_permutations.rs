// q0046_permutations

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for i in nums.iter().cloned() {
            let tmp = vec![i];
            let netp: Vec<i32> = nums.iter().cloned().filter(|&x| x != i).collect();
            let mut tmp2 = Solution::permute(netp);
            if tmp2.len() == 0 {
                ret.push(tmp);
            } else {
                for v in tmp2.iter_mut() {
                    let mut t = tmp.clone();
                    t.append(v);
                    ret.push(t);
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
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            Solution::permute(vec![1, 1, 3])
        );
    }
}
