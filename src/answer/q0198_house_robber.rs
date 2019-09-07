// q0198_house_robber

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut tmp = vec![(0, nums[0])];
        for i in 1..nums.len() {
            let rob_cur = tmp[i - 1].0 + nums[i];
            let not_rob_cur = tmp[i - 1].0.max(tmp[i - 1].1);
            tmp.push((not_rob_cur, rob_cur));
        }
        let l = tmp.last().unwrap();
        l.1.max(l.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
