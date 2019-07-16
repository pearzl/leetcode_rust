// q0075_sort_colors

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        let mut p1 = 0;
        let mut p2 = nums.len() - 1;
        let mut next_0 = p1;
        let mut next_2 = p2;
        while p1 <= p2 {
            // println!(" begin -- p1: {}   p2: {}   0: {}   2: {}", p1, p2, next_0, next_2);
            if nums[p1] == 0 {
                let t = nums[p1];
                nums[p1] = nums[next_0];
                nums[next_0] = t;
                if p1 == nums.len() - 1 {
                    break;
                }
                next_0 += 1;
                p1 += 1;
            } else if nums[p1] == 2 {
                let t = nums[p1];
                nums[p1] = nums[next_2];
                nums[next_2] = t;
                if p2 == 0 {
                    break;
                }
                next_2 -= 1;
                if p2 > next_2 {
                    p2 -= 1;
                }
            } else {
                if p1 == nums.len() - 1 {
                    break;
                }
                p1 += 1;
            }

            if p1 > p2 {
                break;
            }
            // println!("    sort p1: {:?}", nums);
            if nums[p2] == 2 {
                let t = nums[p2];
                nums[p2] = nums[next_2];
                nums[next_2] = t;
                if p2 == 0 {
                    break;
                }
                next_2 -= 1;
                p2 -= 1;
            } else if nums[p2] == 0 {
                let t = nums[p2];
                nums[p2] = nums[next_0];
                nums[next_0] = t;
                if p1 == nums.len() - 1 {
                    break;
                }
                next_0 += 1;
                if p1 < next_0 {
                    p1 += 1;
                }
            } else {
                if p2 == 0 {
                    break;
                }
                p2 -= 1;
            }
            // println!("    sort p2: {:?}", nums);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut arr = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 0, 1, 1, 2, 2]);

        let mut arr = vec![
            2, 0, 2, 1, 1, 0, 0, 1, 0, 2, 0, 2, 0, 1, 0, 2, 1, 1, 1, 0, 2, 0, 2, 2, 2,
        ];
        Solution::sort_colors(&mut arr);
        assert_eq!(
            arr,
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2]
        );

        let mut arr = vec![2, 2];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![2, 2]);

        let mut arr = vec![0, 0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 0]);

        let mut arr = vec![1, 1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![1, 1]);

        let mut arr = vec![2, 1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![1, 2]);

        let mut arr = vec![0, 2, 1];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 1, 2]);

        let mut arr = vec![0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0]);

        let mut arr = vec![2];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![2]);

        let mut arr = vec![2, 1, 0];
        Solution::sort_colors(&mut arr);
        assert_eq!(arr, vec![0, 1, 2]);
    }
}
