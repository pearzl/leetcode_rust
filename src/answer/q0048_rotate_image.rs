// q0048_rotate_image

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut ele_picked = vec![];
        let n = matrix.len();
        let mut start = 0;
        let mut end = n - 1;
        for i in 0..n / 2 {
            for j in start..end {
                ele_picked.push((i, j));
            }
            start += 1;
            end -= 1;
            if start > end {
                break;
            }
        }
        // println!("{:?}", ele_picked);
        for (i, j) in ele_picked.iter().cloned() {
            let (mut dsti, mut dstj) = (i, j);
            let mut tv = matrix[i][j];
            for _ in 0..4 {
                std::mem::swap(&mut dsti, &mut dstj);
                dstj = n - 1 - dstj;
                tv = std::mem::replace(&mut matrix[dsti][dstj], tv);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut m1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut m1);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], m1);

        let mut m2 = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut m2);
        assert_eq!(
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ],
            m2
        );
    }
}
