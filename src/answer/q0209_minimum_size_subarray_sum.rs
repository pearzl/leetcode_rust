// q0209_minimum_size_subarray_sum

struct Solution;

impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }
        let mut min_len = None;
        let mut start = 0;
        let mut end = 1;
        let mut sum = nums[0];
        while start < end {
            if sum >= s {
                if let Some(ml) = min_len {
                    let tlen = end - start;
                    if ml > tlen {
                        min_len = Some(tlen);
                    }
                } else {
                    min_len = Some(end - start)
                }
                if start < end - 1 {
                    sum -= nums[start];
                    start += 1;
                } else if end < len {
                    sum += nums[end];
                    end += 1;
                } else {
                    break;
                }
            } else {
                if end < len {
                    sum += nums[end];
                    end += 1;
                } else {
                    break;
                }
            }
        }
        if let Some(t) = min_len {
            t as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(2, Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    }
}
