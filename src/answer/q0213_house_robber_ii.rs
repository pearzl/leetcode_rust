// q0213_house_robber_ii

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        } else if len <= 2 {
            return nums.into_iter().max().unwrap();
        }

        let max1 = {
            // rob first
            let mut max_money = vec![(0, 0); len];
            // max_money[1] = (0, 0);
            for i in 2..len - 1 {
                let t = max_money[i - 1];
                let rob_cur = nums[i] + t.1;
                let not_rob_cur = t.0.max(t.1);
                max_money[i] = (rob_cur, not_rob_cur);
            }
            let t = max_money[len - 2];
            t.0.max(t.1) + nums[0]
        };
        let max2 = {
            // not rob first
            let mut max_money = vec![(0, 0); len];
            for i in 1..len {
                let t = max_money[i - 1];
                let rob_cur = nums[i] + t.1;
                let not_rob_cur = t.0.max(t.1);
                max_money[i] = (rob_cur, not_rob_cur);
            }
            let t = max_money[len - 1];
            t.0.max(t.1)
        };
        max1.max(max2)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert_eq!( 3, Solution::rob(vec![2,3,2]));
        // assert_eq!( 4, Solution::rob(vec![1,2,3,1]));
        // assert_eq!( 1, Solution::rob(vec![1]));
        // assert_eq!( 3, Solution::rob(vec![1,1,1,2]));
        // assert_eq!( 41, Solution::rob(vec![1,1,3,6,7,10,7,1,8,5,9,1,4,4,3]));
        assert_eq!(103, Solution::rob(vec![1, 3, 1, 3, 100]));
    }
}
