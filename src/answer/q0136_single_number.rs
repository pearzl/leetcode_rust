// q0136_single_number

struct Solution;

// use std::collections::HashSet;
// impl Solution {
//     pub fn single_number(nums: Vec<i32>) -> i32 {
//         let mut set = HashSet::new();
//         for i in nums.into_iter() {
//             if set.contains(&i) {
//                 set.remove(&i);
//             }else {
//                 set.insert(i);
//             }
//         }
//         set.into_iter().next().unwrap()
//     }
// }

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in nums.into_iter() {
            ret ^= i;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}
