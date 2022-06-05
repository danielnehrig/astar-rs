use std::usize;

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref DIAG_BONUS: f32 = 1.4;
    static ref BASE_G_COST: i32 = 10;
    static ref START_INDICATOR: usize = 8;
    static ref END_INDICATOR: usize = 9;
}

fn gen_blockade() -> usize {
    if thread_rng().gen_ratio(1, 7) {
        1
    } else {
        0
    }
}

fn gen_range(x: usize) -> usize {
    thread_rng().gen_range(x..13)
}

fn gen_board(height: usize, width: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for y in 0..height {
        let mut new_vec: Vec<usize> = vec![];
        for x in 0..width {
            new_vec.push(gen_blockade());
        }
        result.push(new_vec);
    }
    result[0][0] = *START_INDICATOR; // start indicator
    result[height - 1][width - 1] = *END_INDICATOR; // end indicator
    return result;
}

fn draw_board(board: Vec<Vec<usize>>) -> () {
    for x in &board {
        println!("{:?}", x);
    }
}

#[derive(Debug)]
struct AStar {
    start: Node,
    end: Node,
    board: Vec<Vec<usize>>,
    solved_path: Vec<usize>,
}

impl Default for AStar {
    fn default() -> Self {
        let height = gen_range(6);
        let width = gen_range(8);
        Self {
            board: gen_board(height, width),
            solved_path: Vec::new(),
            start: Node { x: 0, y: 0 },
            end: Node {
                x: height - 1,
                y: width - 1,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    pub fn get_f_cost(self, node: Node) {
        let up_down_diff = node.x as i32 - self.x as i32;
        let left_right_diff = node.y as i32 - self.y as i32;
        let diff_calc = left_right_diff - up_down_diff;
        let result = ((up_down_diff * *BASE_G_COST) as f32 * *DIAG_BONUS)
            + (diff_calc * *BASE_G_COST) as f32;
        println!("{}", result);
    }
    pub fn get_h_cost(self, board: Vec<Vec<usize>>) {
        unimplemented!();
    }
    pub fn get_cost(self, board: Vec<Vec<usize>>) {
        unimplemented!();
    }
}

impl AStar {
    pub fn solve(self) -> () {
        unimplemented!();
    }

    pub fn gen_surrounding() {
        unimplemented!();
    }
}

fn main() {
    let a_star = AStar::default();
    let test_node = Node {
        x: a_star.end.x - 2,
        y: a_star.end.y - 5,
    };
    let mut board_clone = a_star.board.clone();
    board_clone[test_node.x][test_node.y] = 99;
    draw_board(board_clone);
    test_node.get_f_cost(a_star.end.clone());
    a_star.solve();
}
