// q0217_contains_duplicate

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::with_capacity(nums.len());
        for i in nums {
            if !set.insert(i) {
                return true;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert_eq!(false, Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert_eq!(
            true,
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
        );
    }
}
