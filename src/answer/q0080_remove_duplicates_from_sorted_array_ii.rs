// q0080_remove_duplicates_from_sorted_array_ii

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut k = 1; // next exchange pos
        let mut dupk = 1; // how many duplicate of nums[k-1] before k
        for i in 1..nums.len() {
            // println!("i:num[{}]={}-- k: {}-- dupk: {}", i, nums[i], k, dupk);
            if nums[i] == nums[k - 1] {
                if dupk < 2 {
                    dupk += 1;
                    nums[k] = nums[i];
                    k += 1;
                }
            } else {
                // if dupk < 2 {
                //     nums[k] = nums[i];
                //     k += 1;
                //     dupk = 1;
                // }else {
                nums[k] = nums[i];
                k += 1;
                dupk = 1;
                // }
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut input = vec![1, 1, 1, 2, 2, 3];
        let r = Solution::remove_duplicates(&mut input);
        assert_eq!(vec![1, 1, 2, 2, 3], &input[0..r as usize]);

        let mut input = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let r = Solution::remove_duplicates(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 3, 3], &input[0..r as usize]);

        let mut input = vec![0];
        let r = Solution::remove_duplicates(&mut input);
        assert_eq!(vec![0], &input[0..r as usize]);
    }
}
