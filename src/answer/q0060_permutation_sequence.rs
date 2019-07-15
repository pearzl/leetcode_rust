// q0060_permutation_sequence

struct Solution;

const FACTORIAL: [i32; 9] = [1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const NUMS: [u8; 9] = [b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut picked_num = vec![];
        let mut k = k - 1;
        let n = n as usize;
        for round in (0..n - 1).rev() {
            let index = k / FACTORIAL[round];
            // println!("index{}  k{} round{}", index, k, round);
            Solution::pick_num(&mut picked_num, index, n);
            k %= FACTORIAL[round];
        }
        Solution::pick_num(&mut picked_num, 0, n);
        String::from_utf8(picked_num).unwrap()
    }

    pub fn pick_num(picked_num: &mut Vec<u8>, index: i32, n: usize) {
        // print!("pick {}, already picked {:?}", index, picked_num );
        let mut count = 0;
        for i in 0..n {
            let num = &NUMS[i];
            if picked_num.contains(num) {
                continue;
            }
            if count == index {
                // println!("  picked {}", *num);
                picked_num.push(*num);
                break;
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(String::from("1"), Solution::get_permutation(1, 1));
        assert_eq!(String::from("12"), Solution::get_permutation(2, 1));
        assert_eq!(String::from("21"), Solution::get_permutation(2, 2));
        assert_eq!(String::from("123"), Solution::get_permutation(3, 1));
        assert_eq!(String::from("132"), Solution::get_permutation(3, 2));
        assert_eq!(String::from("213"), Solution::get_permutation(3, 3));
        assert_eq!(String::from("231"), Solution::get_permutation(3, 4));
        assert_eq!(String::from("312"), Solution::get_permutation(3, 5));
        assert_eq!(String::from("321"), Solution::get_permutation(3, 6));
    }

}
