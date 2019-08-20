// q0153_find_minimum_in_rotated_sorted_array

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        Solution::find(&nums)
    }

    fn find(nums: &[i32]) -> i32 {
        let l = nums.len();
        if l == 1 {
            return nums[0];
        } else if l == 2 {
            return nums[0].min(nums[1]);
        }
        let mid_i = l / 2;
        if nums[mid_i - 1] < nums[mid_i] && nums[mid_i] < nums[mid_i + 1] {
            let t1 = Solution::find(&nums[0..mid_i]);
            let t2 = Solution::find(&nums[mid_i..]);
            return t1.min(t2);
        } else if nums[mid_i - 1] > nums[mid_i] {
            return nums[mid_i];
        } else if nums[mid_i] > nums[mid_i + 1] {
            return nums[mid_i + 1];
        } else {
            panic!(1234567890)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
        assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    }
}
