// q0167_two_sum_ii_input_array_is_sorted

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;
        while i < j {
            let sum = numbers[i] + numbers[j];
            if sum == target {
                break;
            } else if sum < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return vec![(i + 1) as i32, (j + 1) as i32];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
