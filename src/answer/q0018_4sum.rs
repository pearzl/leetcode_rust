// q0018_4sum

struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = vec![];
        for (i1, x1) in nums.iter().enumerate() {
            for (i2, x2) in nums.iter().enumerate() {
                if i2 <= i1 {
                    continue;
                }
                let mut i3 = i2 + 1;
                let mut i4 = nums.len() - 1;
                while i3 < i4 {
                    let sum = x1 + x2 + nums[i3] + nums[i4];
                    if sum == target {
                        let mut v = vec![*x1, *x2, nums[i3], nums[i4]];
                        v.sort_unstable();
                        if !ret.contains(&v) {
                            ret.push(v);
                        }
                        i3 += 1;
                        i4 -= 1;
                    } else if sum < target {
                        i3 += 1;
                    } else {
                        i4 -= 1;
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
    use crate::util;

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0)),
            util::vec_2_set(vec![
                vec![-1, 0, 0, 1],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2]
            ])
        )
    }
}
