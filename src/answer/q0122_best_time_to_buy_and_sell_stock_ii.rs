// q0122_best_time_to_buy_and_sell_stock_ii

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
        profit.iter().filter(|&&x| x > 0).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }
}
