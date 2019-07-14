// q0051_n_queens 


struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        if n == 0 {
            return vec![vec![]]
        }
        let mut ret  = vec![];
        let mut map = vec![];
        Solution::solve(n, &mut map, &mut ret);
        // println!("answer: {:?}", ret);

        let ans: Vec<Vec<String>> = ret.into_iter().map(|na| {
            let mut ca = vec![];
            for i in na.iter() {
                let mut t = vec![b'.';n as usize];
                t[*i as usize] = b'Q';
                ca.push(String::from_utf8(t).unwrap());
            }
            ca
        }).collect();
        // println!("result: {:?}", ans);
        ans
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
        let ept: Vec<Vec<String>> = vec![vec![]];
        assert_eq!( Solution::solve_n_queens(0), ept);
        assert_eq!( Solution::solve_n_queens(1), vec![vec![String::from("Q")]]);
        assert_eq!( Solution::solve_n_queens(2), ept);
        assert_eq!( Solution::solve_n_queens(3), ept);
        // assert_eq!( Solution::solve_n_queens(8), vec![vec![String::new()]]);
    }

    #[test]
    #[ignore]
    fn is_attacked() {
        assert_eq!(Solution::is_attacked((0,0), (0,0), 10), true);
        assert_eq!(Solution::is_attacked((0,1), (1,0), 10), true);
        assert_eq!(Solution::is_attacked((0,1), (2,0), 10), false);
        assert_eq!(Solution::is_attacked((3,3), (2,3), 10), true);
        assert_eq!(Solution::is_attacked((3,3), (4,4), 10), true);
        assert_eq!(Solution::is_attacked((5,5), (4,4), 10), true);
        assert_eq!(Solution::is_attacked((3,3), (2,4), 10), true);
        assert_eq!(Solution::is_attacked((3,3), (4,2), 10), true);
    }

    #[test] 
    #[ignore]
    fn valid_pos() {
        assert_eq!(Solution::valid_pos(&vec![], 8), vec![0,1,2,3,4,5,6,7]);
        assert_eq!(Solution::valid_pos(&vec![1,4,6], 10), vec![0,3,8,9]);
        assert_eq!(Solution::valid_pos(&vec![7,2,4,1], 10), vec![8,9]);
    }
}

