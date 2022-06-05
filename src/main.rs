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

fn gen_board(width: usize, height: usize) -> Vec<Vec<usize>> {
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
    board: Vec<Vec<usize>>,
    solved_path: Vec<usize>,
}

impl Default for AStar {
    fn default() -> Self {
        Self {
            board: gen_board(gen_range(5), gen_range(7)),
            solved_path: Vec::new(),
        }
    }
}

fn main() {
    let a_star = AStar::default();
    draw_board(a_star.board.clone());
}
