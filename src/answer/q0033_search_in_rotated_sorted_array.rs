// q0033_search_in_rotated_sorted_array 


struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let nlen = nums.len();
        if nlen == 0 {
            return -1;
        }
        if nums[0] == target {
            return 0
        }else {
            if nlen == 1 {
                return -1;
            }else {
                return Solution::right_search(1, nlen-1, nums[0], target, &nums);
            }
        }
    }

    fn right_search(from: usize, to: usize, min: i32, target: i32, nums: &Vec<i32>) -> i32 {
        let mid_i = (from + to) / 2;
        let mid = &nums[mid_i];
        // println!("{},{},{},{},{}", from, to, min, mid_i, *mid);
        if *mid == target {
            return mid_i as i32
        }
        if min < target && target < *mid {
            match &nums[from..mid_i].binary_search(&target){
                Ok(n) => return (n+from) as i32,
                Err(_) => return -1
            }
        }else if min < *mid && *mid < target {
            if mid_i+1 > to {
                return -1
            }
            return Solution::right_search(mid_i+1, to, *mid, target, nums);
        }else if target < min && min < *mid {
            if mid_i+1 > to {
                return -1
            }
            return Solution::right_search(mid_i+1, to, *mid, target, nums);
        }else if target < *mid && *mid < min {
            if mid_i-1 < from {
                return -1
            }
            return Solution::right_search(from, mid_i-1, min, target, nums);
        }else if *mid < target && target < min {
            if mid_i+1 > to {
                return -1
            }
            match &nums[mid_i+1..=to].binary_search(&target) {
                Ok(n) => return (n+mid_i+1) as i32,
                Err(_) => return -1
            }
        }else if *mid < min && min < target {
            if mid_i+1 <= to {
                if let Ok(n) = &nums[mid_i+1..=to].binary_search(&target) {
                    return  (n+mid_i+1) as i32
                }
            }
            if mid_i-1 >= from {
                return Solution::right_search(from, mid_i-1, min, target, nums);
            }
            return -1
        }
        panic!("uncover")
    }
    
    
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( 4, Solution::search(vec![4,5,6,7,0,1,2], 0));
        assert_eq!( -1, Solution::search(vec![4,5,6,7,0,1,2], 3));
        assert_eq!( -1, Solution::search(vec![4], 3));
        assert_eq!( -1, Solution::search(vec![], 3));
        assert_eq!( 0, Solution::search(vec![3], 3));
        assert_eq!( 1, Solution::search(vec![4,3], 3));
        assert_eq!( 0, Solution::search(vec![3,4], 3));
        assert_eq!( -1, Solution::search(vec![1,3], 4));
        assert_eq!( 1, Solution::search(vec![1,3,5], 3));
        assert_eq!( 2, Solution::search(vec![5,1,3], 3));
        assert_eq!( 4, Solution::search(vec![4,5,6,7,0,1,2], 0));
        assert_eq!( 4, Solution::search(vec![2,3,4,5,1], 1));
        assert_eq!( 1, Solution::search(vec![8,9,2,3,4], 9));
    }
}

