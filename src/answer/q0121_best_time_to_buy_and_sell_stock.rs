// q0121_best_time_to_buy_and_sell_stock

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
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
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
