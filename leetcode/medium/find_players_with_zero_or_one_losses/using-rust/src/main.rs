use std::vec::Vec;

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut max_id: i32 = -1;
        let mut players = vec![-1; 100001];
        let mut won_all_players: Vec<i32> = vec![];
        let mut lose_once_players: Vec<i32> = vec![];

        for i in 0..matches.len() {
            // println!("{:?}", matches[i]);
            if let [w, l] = matches[i][0..matches[i].len()] {
                if w > max_id {
                    max_id = w;
                }
                if l > max_id {
                    max_id = l;
                }
                // print!("[{}-{}]\t",w,l);
                if players[w as usize] == -1 {
                    players[w as usize] = 0
                }

                if players[l as usize] == -1 {
                    players[l as usize] = 1
                } else {
                    players[l as usize] += 1
                }
            }
        }

        for i in 0..=max_id {
            if players[i as usize] == 0 {
                won_all_players.push(i);
            }
            if players[i as usize] == 1 {
                lose_once_players.push(i)
            }
        }

        vec![won_all_players, lose_once_players]
    }
}

fn main() {
    /*
     * Test case 1:
     * matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
     * expected = [[1,2,10],[4,5,7,8]]
     */
    println!(
        "{:?}",
        Solution::find_winners(vec![
            vec![1, 3],
            vec![2, 3],
            vec![3, 6],
            vec![5, 6],
            vec![5, 7],
            vec![4, 5],
            vec![4, 8],
            vec![4, 9],
            vec![10, 4],
            vec![10, 9],
        ])
    );

    /*
     * Test case 2:
     * matches = [[2, 3], [1, 3], [5, 4], [6, 4]]
     * expected = [[1,2,10],[4,5,7,8]]
     */
    println!(
        "{:?}",
        Solution::find_winners(vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]])
    );

    /*
     * Test case 108:
     * matches = [[1,5],[2,5],[2,8],[2,9],[3,8],[4,7],[4,9],[5,7],[6,8]]
     */
    println!(
        "{:?}",
        Solution::find_winners(vec![
            vec![1, 5],
            vec![2, 5],
            vec![2, 8],
            vec![2, 9],
            vec![3, 8],
            vec![4, 7],
            vec![4, 9],
            vec![5, 7],
            vec![6, 8],
        ])
    );
}
