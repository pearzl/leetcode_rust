// q0059_spiral_matrix_ii

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        if n == 0 {
            return vec![];
        }
        let mut ret = Vec::with_capacity(n as usize);
        for _ in 0..n {
            // ret.push(Vec::with_capacity(n as usize)); // leetcode didn't pass this, but it works on my computer
            ret.push(vec![0; n as usize]);
        }
        let mut robot = Robot::new(n as usize, n as usize);
        let mut index = 1;
        while let Some((x, y)) = robot.next() {
            ret[x][y] = index;
            index += 1;
        }
        ret
    }
}

enum Direction {
    U,
    D,
    L,
    R,
    E,
}

struct Robot {
    boundary: (usize, usize, usize, usize),
    direction: Direction,
    cur_pos: (usize, usize),
}

impl Robot {
    pub fn new(line: usize, row: usize) -> Robot {
        Robot {
            boundary: (0, row - 1, line - 1, 0),
            direction: Direction::R,
            cur_pos: (0, 0),
        }
    }
}

impl std::iter::Iterator for Robot {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_pos_line = self.cur_pos.0;
        let mut new_pos_row = self.cur_pos.1;
        match self.direction {
            Direction::U => {
                if self.cur_pos.0 < self.boundary.0 + 1 {
                    if new_pos_row + 1 <= self.boundary.1 {
                        self.direction = Direction::R;
                        self.boundary.3 += 1;
                        new_pos_row += 1;
                    }
                } else {
                    new_pos_line -= 1;
                }
            }
            Direction::D => {
                if self.cur_pos.0 + 1 > self.boundary.2 {
                    if new_pos_row >= self.boundary.3 + 1 {
                        self.direction = Direction::L;
                        self.boundary.1 -= 1;
                        new_pos_row -= 1;
                    }
                } else {
                    new_pos_line += 1;
                }
            }
            Direction::L => {
                if self.cur_pos.1 < self.boundary.3 + 1 {
                    if new_pos_line >= self.boundary.0 + 1 {
                        self.direction = Direction::U;
                        self.boundary.2 -= 1;
                        new_pos_line -= 1;
                    }
                } else {
                    new_pos_row -= 1;
                }
            }
            Direction::R => {
                if self.cur_pos.1 + 1 > self.boundary.1 {
                    if new_pos_line + 1 <= self.boundary.2 {
                        self.direction = Direction::D;
                        self.boundary.0 += 1;
                        new_pos_line += 1;
                    }
                } else {
                    new_pos_row += 1;
                }
            }
            Direction::E => return None,
        }
        if new_pos_line == self.cur_pos.0 && new_pos_row == self.cur_pos.1 {
            self.direction = Direction::E;
        }
        return Some(std::mem::replace(
            &mut self.cur_pos,
            (new_pos_line, new_pos_row),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::generate_matrix(0));
        assert_eq!(vec![vec![1]], Solution::generate_matrix(1));
        assert_eq!(vec![vec![1, 2], vec![3, 4]], Solution::generate_matrix(2));
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
        // assert_eq!( vec![], Solution::generate_matrix(4));
        // assert_eq!( vec![], Solution::generate_matrix(5));
    }
}
