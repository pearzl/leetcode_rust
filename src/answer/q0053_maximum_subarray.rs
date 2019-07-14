// q0053_maximum_subarray

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Solution::max_sum(&nums)
    }

    pub fn max_sum(nums: &[i32]) -> i32 {
        // println!("{:?}", nums);
        let nlen = nums.len();
        if nlen == 0 {
            return 0;
        } else if nlen == 1 {
            return nums[0];
        } else {
            let r = Solution::max_sum(&nums[nlen / 2..]);
            let l = Solution::max_sum(&nums[0..nlen / 2]);
            let mut rmax = nums[nlen / 2];
            let mut rsum = rmax;
            for i in nlen / 2 + 1..nums.len() {
                rsum = rsum + nums[i];
                if rsum >= rmax {
                    rmax = rsum;
                }
            }
            let mut lmax = nums[nlen / 2 - 1];
            let mut lsum = lmax;
            for i in 1..nlen / 2 {
                lsum = lsum + nums[nlen / 2 - i - 1];
                if lsum >= lmax {
                    lmax = lsum;
                }
            }
            // println!("lmax: {}  rmax: {} r: {} l: {}", lmax, rmax, r, l);
            return (lmax + rmax).max(r).max(l);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
}
