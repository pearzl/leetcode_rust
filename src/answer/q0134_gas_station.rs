// q0134_gas_station

struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let gas_change: Vec<i32> = gas
            .into_iter()
            .zip(cost.into_iter())
            .map(|x| x.0 - x.1)
            .collect();
        let sum: i32 = gas_change.iter().sum();
        if sum < 0 {
            return -1;
        }
        let l = gas_change.len();
        for start_from in 0..l {
            if gas_change[start_from] < 0 {
                continue;
            }
            let mut car_gas = 0;
            let mut pos = start_from;
            loop {
                car_gas += gas_change[pos];
                if car_gas < 0 {
                    break;
                }
                if pos == l - 1 {
                    pos = 0;
                } else {
                    pos += 1;
                }
                if pos == start_from {
                    return start_from as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
        assert_eq!(
            4,
            Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1])
        );
    }
}
