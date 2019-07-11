// q0031_next_permutation 


struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        if len < 2 {
            return 
        }
        let mut i = len - 2;
        let mut j = len - 1;
        loop{
            if nums[i] < nums[j] {
                nums.swap(i, j);
                break;
            }
            if j <= i {
                if i == 0 {
                    nums.sort();
                    return;
                }else {
                    i -= 1;
                    j = len - 1;
                }
            }else {
                j -= 1;
            }
        }
        nums[i+1..].sort();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut v1 = vec![1,2,3];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![1,3,2]);

        let mut v1 = vec![3,2,1];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![1,2,3]);

        let mut v1 = vec![1,1,5];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![1,5,1]);

        let mut v1 = vec![];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![]);

        let mut v1 = vec![11];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![11]);

        let mut v1 = vec![1,3,2];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![2,1,3]);

        let mut v1 = vec![4,5,6,5,4,5,3,1];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![4,5,6,5,5,1,3,4]);
        
        let mut v1 = vec![4,2,0,2,3,2,0];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![4,2,0,3,0,2,2]);

        let mut v1 = vec![4,2,0,1,3,2,0];
        Solution::next_permutation(&mut v1);
        assert_eq!( v1 , vec![4,2,0,2,0,1,3]);
    }
}

