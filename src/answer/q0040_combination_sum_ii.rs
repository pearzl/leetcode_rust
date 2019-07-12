// q0040_combination_sum_ii 


struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut ret = vec![];
        let mut tmp = vec![];
        Solution::solve(&candidates, target, &mut tmp, &mut ret);
        let mut t = vec![];
        while let Some(mut v) = ret.pop() {
            v.sort_unstable();
            if !t.contains(&v) {
                t.push(v)
            }
        }
        t
    }

    fn solve(cddt: &[i32], target: i32, tmp: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        for (index, i) in cddt.iter().enumerate() {
            tmp.push(*i);
            let sum: i32 = tmp.iter().sum();
            if sum == target {
                ret.push(tmp.clone());
                tmp.pop();
                return
            }else if sum > target {
                tmp.pop();
                return
            }else {
                Solution::solve(&cddt[index+1..], target, tmp, ret);
                tmp.pop();
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combination_sum2(vec![10,1,2,7,6,1,5], 8) , vec![
        vec![1, 7],
        vec![1, 2, 5],
        vec![2, 6],
        vec![1, 1, 6]
] );
    }
}

