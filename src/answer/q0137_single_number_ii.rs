// q0137_single_number_ii

struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().cloned().collect();
        let xxxy: i32 = nums.iter().sum();
        let xy: i32 = set.iter().sum();
        let xx = xxxy - xy;
        println!("{}-{}-{}-{}", xxxy, xy, xx, xy - xx / 2);
        xy - xx / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(3, Solution::single_number(vec![2, 2, 3, 2]));
    }
}
