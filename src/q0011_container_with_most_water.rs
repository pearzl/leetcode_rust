// q0011_container_with_most_water

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        for i in 0..height.len() {
            let mut j = i + 1;
            while j < height.len() {
                let area = (j - i) as i32 * height[i].min(height[j]);
                max_area = max_area.max(area);
                j += 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
