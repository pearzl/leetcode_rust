// q0123_best_time_to_buy_and_sell_stock_iii 


struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..prices.len() {
            let t1 = Solution::max_profit_1(&prices[0..=i]);
            let t2 = Solution::max_profit_1(&prices[i..]);
            max = max.max(t1+t2);
            // println!("{}: {}-{}-{}", i, t1, t2, max);
        }
        max
    }

    
    pub fn max_profit_1(prices: &[i32]) -> i32 {
        let len = prices.len();
        if len <= 1 {
            return 0;
        }
        let mut profit = vec![0; len - 1];
        for i in 0..len - 1 {
            profit[i] = prices[i + 1] - prices[i];
        }
        let mut max_profit_days = vec![0; len - 1];
        max_profit_days[0] = profit[0];
        for i in 1..len - 1 {
            max_profit_days[i] = profit[i].max(profit[i] + max_profit_days[i - 1]);
        }
        let m = max_profit_days.into_iter().max().unwrap();
        if m < 0 {
            return 0;
        } else {
            return m;
        }
    }

    
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( 6, Solution::max_profit(vec![3,3,5,0,0,3,1,4]));
        assert_eq!( 4, Solution::max_profit(vec![1,2,3,4,5]));
        assert_eq!( 13, Solution::max_profit(vec![1,2,4,2,5,7,2,4,9,0]));
    }
}

