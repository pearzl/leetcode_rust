// q0215_kth_largest_element_in_an_array

struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let len = nums.len();
        for i in 0..k {
            for j in 1..len - i {
                if nums[j - 1] > nums[j] {
                    let t = nums[j - 1];
                    nums[j - 1] = nums[j];
                    nums[j] = t;
                }
            }
        }
        nums[len - k]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
        assert_eq!(
            4,
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)
        );
    }
}
