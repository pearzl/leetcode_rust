// q0189_rotate_array

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        if l == 0 {
            return;
        }
        let k = k as usize % l;
        let tmp = Vec::from(&nums[l - k..]);
        for i in (k..l).rev() {
            nums[i] = nums[i - k];
        }
        for (i, v) in tmp.iter().enumerate() {
            nums[i] = *v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut v, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], v);

        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate(&mut v, 2);
        assert_eq!(vec![3, 99, -1, -100], v);
    }
}
