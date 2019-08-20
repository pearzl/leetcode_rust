// q0164_maximum_gap

struct Solution;

impl Solution {
    pub fn maximum_gap(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let build_buf = || {
            let mut r = Vec::with_capacity(10);
            for _ in 0..10 {
                r.push(vec![]);
            }
            r
        };
        let mut swap_nums = Vec::with_capacity(nums.len());
        for i in 0..10 {
            let mut base = 1;
            for _ in 0..i {
                base *= 10;
            }
            let mut buf = build_buf();
            for n in nums.iter().cloned() {
                let bi = n / base % 10;
                buf[bi as usize].push(n);
            }
            swap_nums.clear();
            for vi in buf.iter() {
                for n in vi.iter().cloned() {
                    swap_nums.push(n);
                }
            }
            std::mem::swap(&mut swap_nums, &mut nums);
        }
        let mut max_gap = 0;
        for i in 0..nums.len() - 1 {
            max_gap = max_gap.max(nums[i + 1] - nums[i]);
        }
        max_gap
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(0, Solution::maximum_gap(vec![1, 1]));
        assert_eq!(3, Solution::maximum_gap(vec![3, 6, 9, 1]));
        assert_eq!(
            i32::max_value(),
            Solution::maximum_gap(vec![0, i32::max_value()])
        );
    }
}
