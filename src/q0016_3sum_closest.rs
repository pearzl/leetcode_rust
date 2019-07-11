// q0016_3sum_closest

struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() < 3 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut dsdc = i32::max_value();
        let mut sum = 0;
        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let n = nums[i] + nums[j] + nums[k];
                if n == target {
                    return n;
                } else if n >= target {
                    let td = n - target;
                    if td < dsdc {
                        dsdc = td;
                        sum = n;
                    }
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let n = nums[i] + nums[j] + nums[k];
                if n == target {
                    return n;
                } else if n <= target {
                    let td = target - n;
                    if td < dsdc {
                        dsdc = td;
                        sum = n;
                    }
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::three_sum_closest(vec![-1, 2, 1, -4], 1));
    }
}
