// q0204_count_primes

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut primes = vec![2];
        fn is_primes(x: i32, primes: &[i32]) -> bool {
            let mut ret = true;
            for i in 0..primes.len() {
                if x % primes[i] == 0 {
                    ret = false;
                    break;
                }
                if primes[i] * primes[i] > x {
                    break;
                }
            }
            ret
        };
        for nx in 3..n {
            if is_primes(nx, &primes) {
                primes.push(nx);
            }
        }
        primes.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::count_primes(10));
        assert_eq!(3, Solution::count_primes(7));
        assert_eq!(0, Solution::count_primes(2));
    }
}
