// q0026_remove_duplicates_from_sorted_array

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let mut a = 1;
        let mut b = 0;
        while a < nums.len() {
            while nums[b] >= nums[a] {
                a += 1;
                if a >= nums.len() {
                    return (b + 1) as i32;
                }
            }
            b += 1;
            nums[b] = nums[a];
        }
        (b + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut v1 = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut v1), 2);
        assert_eq!(&v1[0..2], &(vec![1, 2][..]));

        let mut v1 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut v1), 5);
        assert_eq!(&v1[0..5], &(vec![0, 1, 2, 3, 4][..]));
    }
}
