// q0034_find_first_and_last_position_of_element_in_sorted_array

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(n) => {
                let mut p1 = n as i32;
                let mut p2 = n as i32;
                while p1 >= 0 && nums[p1 as usize] == target {
                    p1 -= 1
                }
                while p2 <= (nums.len() - 1) as i32 && nums[p2 as usize] == target {
                    p2 += 1;
                }
                vec![p1 + 1, p2 - 1]
            }
            Err(_) => vec![-1, -1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }
}
