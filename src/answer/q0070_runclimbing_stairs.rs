// q0070_runclimbing_stairs

struct Solution;

// impl Solution {
//     pub fn climb_stairs(n: i32) -> i32 {
//         if n == 1 {
//             return 1;
//         } else if n == 2 {
//             return 2;
//         } else {
//             return Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
//         }
//     }
// }

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut buf = vec![0; n as usize];
        Solution::climb(n as usize, &mut buf)
    }
    fn climb(n: usize, buf: &mut Vec<i32>) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else {
            if buf[n - 1] == 0 {
                buf[n - 1] = Solution::climb(n - 1, buf) + Solution::climb(n - 2, buf);
            }
            return buf[n - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let ret = [
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
            10946,
        ];
        for (i, o) in (1..=20).into_iter().zip(ret.into_iter().cloned()) {
            assert_eq!(o, Solution::climb_stairs(i));
        }
    }
}
