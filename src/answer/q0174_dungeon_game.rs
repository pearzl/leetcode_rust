// q0174_dungeon_game

struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let l = dungeon.len();
        let r = dungeon[0].len();
        let mut min_hp = vec![vec![0; r]; l];
        min_hp[l - 1][r - 1] = if dungeon[l - 1][r - 1] < 0 {
            1 - dungeon[l - 1][r - 1]
        } else {
            1
        };
        let cal_min_hp = |cur_change: i32, min_hp_need: i32| {
            if cur_change < min_hp_need {
                min_hp_need - cur_change
            } else {
                1
            }
        };
        for i in (0..l - 1).rev() {
            min_hp[i][r - 1] = cal_min_hp(dungeon[i][r - 1], min_hp[i + 1][r - 1]);
        }
        for i in (0..r - 1).rev() {
            min_hp[l - 1][i] = cal_min_hp(dungeon[l - 1][i], min_hp[l - 1][i + 1]);
        }
        for line in (0..l - 1).rev() {
            for row in (0..r - 1).rev() {
                let t1 = cal_min_hp(dungeon[line][row], min_hp[line + 1][row]);
                let t2 = cal_min_hp(dungeon[line][row], min_hp[line][row + 1]);
                min_hp[line][row] = t1.min(t2);
            }
        }
        min_hp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ])
        );
    }
}
