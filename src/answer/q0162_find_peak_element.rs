// q0162_find_peak_element

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if let Some(i) = Solution::find_middle_peak_element(&nums) {
            return i as i32;
        } else {
            let l = nums.len();
            if l == 1 {
                return 0;
            }
            if nums[0] >= nums[1] {
                return 0;
            }
            if nums[l - 1] >= nums[l - 2] {
                return (l - 1) as i32;
            }
            panic!(1234567890)
        }
    }

    fn find_middle_peak_element(nums: &[i32]) -> Option<usize> {
        // println!("{:?}", nums);
        let l = nums.len();
        if l <= 2 {
            return None;
        }
        let midi = l / 2;
        if nums[midi] >= nums[midi - 1] && nums[midi] >= nums[midi + 1] {
            return Some(midi);
        } else if nums[midi] >= nums[midi - 1] {
            if let Some(i) = Solution::find_middle_peak_element(&nums[0..midi]) {
                return Some(i);
            }
            if let Some(i) = Solution::find_middle_peak_element(&nums[midi..]) {
                return Some(midi + i);
            }
            return None;
        } else if nums[midi] >= nums[midi + 1] {
            if let Some(i) = Solution::find_middle_peak_element(&nums[0..=midi]) {
                return Some(i);
            }
            if let Some(i) = Solution::find_middle_peak_element(&nums[midi + 1..]) {
                return Some(midi + i + 1);
            }
            return None;
        } else {
            if let Some(i) = Solution::find_middle_peak_element(&nums[0..=midi]) {
                return Some(i);
            }
            if let Some(i) = Solution::find_middle_peak_element(&nums[midi..]) {
                return Some(midi + i);
            }
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // assert!(vec![2].contains(&Solution::find_peak_element(vec![1,2,3,1])));
        // assert!(vec![1,5].contains(&Solution::find_peak_element(vec![1,2,1,3,5,6,4])));
        assert!(vec![1].contains(&Solution::find_peak_element(vec![1, 3, 2, 1])));
    }
}
