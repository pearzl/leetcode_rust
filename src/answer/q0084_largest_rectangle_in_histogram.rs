// q0084_largest_rectangle_in_histogram

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut max_area = vec![];
        for (i, h) in heights.iter().enumerate() {
            let mut p1 = i;
            let mut p2 = i;
            while p1 > 0 {
                p1 -= 1;
                if heights[p1] < *h {
                    p1 += 1;
                    break;
                }
            }
            while p2 < heights.len() - 1 {
                p2 += 1;
                if heights[p2] < *h {
                    p2 -= 1;
                    break;
                }
            }
            let ma = (p2 - p1 + 1) as i32 * *h;
            max_area.push(ma);
        }
        // println!("{:?}", max_area);
        match max_area.iter().max() {
            Some(n) => return *n as i32,
            None => return 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(10, Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]));
        assert_eq!(0, Solution::largest_rectangle_area(Vec::<i32>::new()));
    }
}
