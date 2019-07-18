// q0088_merge_sorted_array 


struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = nums1.len();
        let mut m = m as usize;
        let mut n = n as usize;
        while i != 0 {
            i -= 1;
            if m == 0 {
                n -= 1;
                nums1[i] = nums2[n];
            }else if n == 0 {
                m -= 1;
                nums1[i] = nums1[m];
            }else {
                if nums1[m-1] < nums2[n-1] {
                    n -= 1;
                    nums1[i] = nums2[n];
                }else {
                    m -= 1;
                    nums1[i] = nums1[m];
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        // let mut a1 = vec![1,2,3,0,0,0];
        // let mut a2 = vec![2,5,6];
        // let l2 = a2.len() as i32;
        // Solution::merge(&mut a1, 3, &mut a2, l2);
        // assert_eq!( vec![1,2,2,3,5,6], a1);

        let mut a1 = vec![0];
        let l1 = 0;
        let mut a2 = vec![1];
        let l2 = a2.len() as i32;
        Solution::merge(&mut a1, l1, &mut a2, l2);
        assert_eq!( vec![1], a1);
    }
}

