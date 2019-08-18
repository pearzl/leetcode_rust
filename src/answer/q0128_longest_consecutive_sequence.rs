// q0128_longest_consecutive_sequence

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().cloned().collect();
        let mut max_len = 0;
        for i in nums.iter() {
            if set.contains(&(i - 1)) {
                continue;
            }
            let mut t = i + 1;
            while set.contains(&t) {
                t += 1;
            }
            max_len = max_len.max(t - i);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
    }
}
