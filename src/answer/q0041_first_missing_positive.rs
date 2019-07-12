// q0041_first_missing_positive 


struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let nlen = nums.len();
        if nlen == 0 {
            return 1;
        }
        let mut i = 0;
        while i < nlen {
            let cur_v = nums[i];
            if cur_v == (i+1) as i32{
                i += 1;
                continue;
            }
            if cur_v >= nlen as i32 || cur_v <= 0{
                i += 1;
                continue;
            }
            if cur_v == nums[cur_v as usize -1] {
                i += 1;
                continue;
            }
            let t = nums[i];
            nums[i] = nums[cur_v as usize -1];
            nums[cur_v as usize-1] = t;
        }
        println!("{:?}", nums);
        for (i, &v) in nums.iter().enumerate() {
            let n = (i+1) as i32;
            if n != v {
                return n
            }
        }
        (nlen+1) as i32
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( 1, Solution::first_missing_positive(vec![7,8,9,11,12]));
        assert_eq!( 2, Solution::first_missing_positive(vec![3,4,-1,1]));
        assert_eq!( 3, Solution::first_missing_positive(vec![1,2,0]));
        assert_eq!( 4, Solution::first_missing_positive(vec![1,2,5,7,2,3]));
        assert_eq!( 1, Solution::first_missing_positive(vec![]));
    }
}

