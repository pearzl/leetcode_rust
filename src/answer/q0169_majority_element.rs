// q0169_majority_element

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let cpr = |c| {
            if nums.len() % 2 == 0 {
                c >= nums.len() / 2
            } else {
                c > nums.len() / 2
            }
        };
        for i in nums.iter() {
            if let Some(c) = map.get_mut(i) {
                *c += 1;
                if cpr(*c) {
                    return *i;
                }
            } else {
                map.insert(*i, 1);
                if cpr(1) {
                    return *i;
                }
            }
        }
        panic!(1234567890);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(2, Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
    }
}
