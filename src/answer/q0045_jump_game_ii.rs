// q0045_jump_game_ii 


struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }
        let mut ret = 1;
        let mut cur = 0;
        Solution::r#move(&mut cur, &mut ret, &nums);
        ret
    }

    fn r#move(cur: &mut usize, times: &mut i32, nums: &Vec<i32>) {
        let sight = nums[*cur] as usize;
        if *cur+sight >= nums.len()-1 {
            return 
        }
        let mut next_pos = *cur;
        for i in *cur..=*cur+sight {
            let farthest = i + nums[i] as usize ;
            if farthest > *cur + nums[*cur] as usize {
                *cur = i;
            }
        }
        *times += 1;
        Solution::r#move(cur, times, nums)
    }


}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::jump(vec![2,3,1,1,4]));
        assert_eq!(0, Solution::jump(vec![]));
        assert_eq!(30, Solution::jump(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]));
        assert_eq!(0, Solution::jump(vec![4]));
        assert_eq!(2, Solution::jump(vec![4,10,1,2,9,6,1,1,1,1,1,1,1,1]));
    }
}

