// q0188_best_time_to_buy_and_sell_stock_iv

struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k <= 0 {
            return 0;
        }
        let mut prices_changes = Vec::with_capacity(prices.len());
        let mut accu_changes = 0;
        for i in 1..prices.len() {
            let change = prices[i] - prices[i - 1];
            if accu_changes == 0 {
                if change > 0 {
                    accu_changes = change;
                }
            } else if change == 0 {
                continue;
            } else if (change > 0 && accu_changes > 0) || (change < 0 && accu_changes < 0) {
                accu_changes += change;
            } else if (change < 0 && accu_changes > 0) || (change > 0 && accu_changes < 0) {
                prices_changes.push(accu_changes);
                accu_changes = change;
            } else {
                panic!("cannot reach!");
            }
        }
        if accu_changes > 0 {
            prices_changes.push(accu_changes);
        }
        // println!("{:?}", prices_changes);
        if prices_changes.len() == 1 {
            return prices_changes[0];
        }
        loop {
            let trades_num = prices_changes.len() / 2 + 1;
            if trades_num <= k as usize {
                let r: i32 = prices_changes.iter().filter(|&x| *x > 0).sum();
                break r;
            }
            let min_loss = prices_changes.iter().map(|x| x.abs()).min().unwrap();
            for i in 0..prices_changes.len() {
                if prices_changes[i].abs() == min_loss {
                    if i == 0 {
                        prices_changes.remove(i + 1);
                        prices_changes.remove(i);
                    } else if i == prices_changes.len() - 1 {
                        prices_changes.remove(i);
                        prices_changes.remove(i - 1);
                    } else {
                        prices_changes[i] =
                            prices_changes[i - 1] + prices_changes[i] + prices_changes[i + 1];
                        prices_changes.remove(i + 1);
                        prices_changes.remove(i - 1);
                    }
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
        assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
        assert_eq!(7, Solution::max_profit(2, vec![3, 4, 3, 2, 6, 5, 0, 3]));
        assert_eq!(5, Solution::max_profit(1, vec![6, 1, 6, 4, 3, 0, 2]));
    }
}
