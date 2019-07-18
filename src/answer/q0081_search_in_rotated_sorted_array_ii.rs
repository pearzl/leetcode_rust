// q0081_search_in_rotated_sorted_array_ii

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Solution::find(&nums[..], target)
    }

    fn find(nums: &[i32], target: i32) -> bool {
        // println!("search {} in {:?}", target, nums);
        let nlen = nums.len();
        if nlen == 0 {
            return false;
        } else if nlen == 1 {
            return target == nums[0];
        }
        let mid_i = nums.len() / 2;
        if nums[mid_i] == target {
            // println!("hit");
            return true;
        } else if nums[mid_i] > target {
            if nums[0] < nums[mid_i] {
                // println!("Bsearch in left part || Csearch in right part");
                return nums[0..mid_i].binary_search(&target).is_ok()
                    || Solution::find(&nums[mid_i + 1..], target);
            } else if nums[0] > nums[mid_i] {
                // println!("Csearch in left part");
                return Solution::find(&nums[0..mid_i], target);
            } else {
                return Solution::find(&nums[mid_i + 1..], target)
                    || Solution::find(&nums[0..mid_i], target);
            }
        } else {
            if nums[0] < nums[mid_i] {
                // println!("Csearch in right part");
                return Solution::find(&nums[mid_i + 1..], target);
            } else if nums[0] > nums[mid_i] {
                // println!("Bsearch in right part || Csearch in left part");
                return nums[mid_i + 1..].binary_search(&target).is_ok()
                    || Solution::find(&nums[0..mid_i], target);
            } else {
                return Solution::find(&nums[mid_i + 1..], target)
                    || Solution::find(&nums[0..mid_i], target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 1));
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 2));
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 4));
        assert_eq!(true, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 6));
        assert_eq!(false, Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 7));
        assert_eq!(true, Solution::search(vec![1, 3, 1, 1], 3));
        assert_eq!(true, Solution::search(vec![1, 3, 1, 1], 1));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1], 4));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1], 0));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1], 2));
        assert_eq!(true, Solution::search(vec![1, 3, 1, 1, 1], 3));
        assert_eq!(true, Solution::search(vec![1, 3, 1, 1, 1], 1));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1, 1], 4));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1, 1], 0));
        assert_eq!(false, Solution::search(vec![1, 3, 1, 1, 1], 2));
        assert_eq!(false, Solution::search(vec![2], 7));
        assert_eq!(true, Solution::search(vec![2], 2));
        assert_eq!(false, Solution::search(Vec::<i32>::new(), 2));
    }
}
