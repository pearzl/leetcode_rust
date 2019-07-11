// q0027_remove_element

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut a = 0;
        let mut b = 0;
        while a < nums.len() {
            println!("{}--{}", a, b);
            while nums[a] == val {
                a += 1;
                if a >= nums.len() {
                    return b as i32;
                }
            }
            nums[b] = nums[a];
            a += 1;
            b += 1;
        }
        return b as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut v = vec![3, 2, 2, 3];
        let n = Solution::remove_element(&mut v, 3);
        assert_eq!(2, n);
        assert_eq!(&v[0..n as usize], &vec![2, 2][..]);

        let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let n = Solution::remove_element(&mut v, 2);
        assert_eq!(5, n);
        assert_eq!(&v[0..n as usize], &vec![0, 1, 3, 0, 4][..]);
    }
}
