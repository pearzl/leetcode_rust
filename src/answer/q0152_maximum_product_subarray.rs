// q0152_maximum_product_subarray

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut dp = Vec::with_capacity(nums.len());
        dp.push((nums[0], nums[0]));
        for (i, n) in nums[1..].iter().cloned().enumerate() {
            let t1 = dp[i].0 * n;
            let t2 = dp[i].1 * n;
            dp.push((t1.min(t2).min(n), t1.max(t2).max(n)));
        }
        // println!("{:?}", dp);
        dp.into_iter().map(|x| x.1).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(6, Solution::max_product(vec![2, 3, -2, 4]));
        assert_eq!(0, Solution::max_product(vec![-2, 0, -1]));
    }
}
