// q0052_n_queens_ii 


struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n == 0 {
            return 1
        }
        let mut ret  = vec![];
        let mut map = vec![];
        Solution::solve(n, &mut map, &mut ret);
        ret.len() as i32
        // println!("answer: {:?}", ret);

        // let ans: Vec<Vec<String>> = ret.into_iter().map(|na| {
        //     let mut ca = vec![];
        //     for i in na.iter() {
        //         let mut t = vec![b'.';n as usize];
        //         t[*i as usize] = b'Q';
        //         ca.push(String::from_utf8(t).unwrap());
        //     }
        //     ca
        // }).collect();
        // println!("result: {:?}", ans);
        // ans
    }

    fn solve(n: i32, cur: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        let posp = Solution::valid_pos(cur, n);
        // println!("{:?}", posp);
        if cur.len() == n as usize - 1 {
            if posp.len() != 0 {
                for i in posp.iter() {
                    cur.push(*i);
                    ret.push(cur.clone());
                    cur.pop();
                }
            }
            return 
        }
        for i in posp.iter() {
            cur.push(*i);
            Solution::solve(n, cur, ret);
            cur.pop();
        }
    }

    pub fn valid_pos(cur: &Vec<i32>, n: i32) -> Vec<i32>{
        let n = n as usize;
        let ci = cur.len();
        let mut ret = vec![];
        for cj in 0..n {
            let mut ia = false;
            for (i,j) in cur.iter().enumerate() {
                if Solution::is_attacked((i,*j as usize), (ci, cj), n) {
                    ia = true;
                }
            }
            if !ia {
                ret.push(cj as i32);
            }
        }
        ret
    }

    
    pub fn is_attacked(p1: (usize, usize), p2: (usize, usize), n: usize) -> bool {
        if p1.0 == p2.0 || p1.1 == p2.1 {
            return true;
        }
        for i in 1..=p1.0 {
            if p1.0 - i  == p2.0 {
                if p1.1 + i < n && p1.1 + i  == p2.1 { // -i +i
                    return true;
                }
                if p1.1 >= i && p1.1 - i == p2.1 {  // -i -i
                    return true;
                }
            }
        }
        for i in 1..n-p1.0 {
            if p1.0 + i  == p2.0 {
                if p1.1 + i < n && p1.1 + i  == p2.1 { // +i +i
                    return true;
                }
                if p1.1 >= i && p1.1 - i == p2.1 {  // +i -i
                    return true;
                }
            }
        }
        false
    }

}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!( Solution::total_n_queens(0), 1);
        assert_eq!( Solution::total_n_queens(1), 1);
        assert_eq!( Solution::total_n_queens(2), 0);
        assert_eq!( Solution::total_n_queens(3), 0);
        assert_eq!( Solution::total_n_queens(4), 2);
        assert_eq!( Solution::total_n_queens(5), 10);
        assert_eq!( Solution::total_n_queens(6), 4);
        assert_eq!( Solution::total_n_queens(7), 40);
        assert_eq!( Solution::total_n_queens(8), 92);
    }
}

