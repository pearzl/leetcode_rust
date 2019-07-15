// q0054_spiral_matrix

struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let line = matrix.len();
        if line == 0 {
            return vec![];
        }
        let row = matrix[0].len();
        let mut robot = Robot::new(line, row);
        // robot.map(|(i, j)| matrix[i][j]).cloned().collect()
        let mut ret = vec![];
        while let Some((i, j)) = robot.next() {
            // println!("{:?}", (i, j));
            ret.push(matrix[i][j]);
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
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
    }
}
