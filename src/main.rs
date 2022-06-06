use std::cell::RefCell;
use std::usize;

use colored::Colorize;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

lazy_static! {
    /// the bondues for a diag node
    static ref DIAG_BONUS: f32 = 1.4;
    /// base g cost
    static ref BASE_G_COST: i32 = 10;
    /// indicator for start position
    static ref START_INDICATOR: usize = 8;
    /// indicator for end position
    static ref END_INDICATOR: usize = 9;
    /// the max board size
    static ref MAX_BOARD_SIZE: usize = 13;
    /// the blocked node value
    static ref BLOCKED_NODE: usize = 1;
    /// the free node value
    static ref FREE_NODE: usize = 0;
}

/// random generation of blocked nodes
fn gen_blockade() -> usize {
    if thread_rng().gen_ratio(1, 7) {
        *BLOCKED_NODE
    } else {
        *FREE_NODE
    }
}

fn gen_range(x: usize) -> usize {
    thread_rng().gen_range(x..*MAX_BOARD_SIZE)
}

/// get a random position on the board
/// used to create start and end points
fn get_rand_pos(height: usize, width: usize) -> (usize, usize) {
    (
        thread_rng().gen_range(0..height),
        thread_rng().gen_range(0..width),
    )
}

/// generate a 2d board
fn gen_board(
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for _ in 0..height {
        let mut new_vec: Vec<usize> = vec![];
        for _ in 0..width {
            new_vec.push(gen_blockade());
        }
        result.push(new_vec);
    }
    result[start.0][start.1] = *START_INDICATOR; // start indicator
    result[end.0][end.1] = *END_INDICATOR; // end indicator
    return result;
}

/// draw the board in stdout
fn draw_board(board: Vec<Vec<usize>>, selected: Vec<(usize, usize)>) -> () {
    print!("     ");
    for y in 0..board.clone()[0].len() {
        print!("{} ", y);
    }
    println!("");
    for (i, x) in board.clone().iter().enumerate() {
        if i < 10 {
            print!("{}  [ ", i);
        } else {
            print!("{} [ ", i);
        }
        for (j, y) in x.clone().into_iter().enumerate() {
            // draw end in red
            let mut is_selected = false;
            for (x1, y1) in &selected {
                if i == *x1 && j == *y1 {
                    is_selected = true;
                }
            }
            if y == 9 {
                print!("{} ", format!("{}", y).bold().red());
            // draw start in green
            } else if y == 8 {
                print!("{} ", format!("{}", y).bold().green());
            // debug output yellow :)
            } else if is_selected {
                print!("{} ", format!("{}", y).bold().yellow());
            } else {
                print!("{} ", y);
            }
        }
        print!("]");
        println!("");
    }
}

#[derive(Debug)]
struct AStar {
    /// the start node
    start: Node,
    /// the end node
    end: Node,
    /// the game board
    board: RefCell<Vec<Vec<usize>>>,
    /// the list of the solved path
    solved_path: RefCell<Vec<Node>>,
    /// the current neighbours
    neighbours_list: RefCell<Vec<Node>>,
}

impl Default for AStar {
    fn default() -> Self {
        let height = gen_range(6); // board height
        let width = gen_range(8); // board width
        let start = get_rand_pos(height, width); // start position tuple
        let end = get_rand_pos(height, width); // end position tuple
        println!(
            "height: {} width: {} start: {:?} end: {:?}",
            height, width, start, end
        ); // random gen numbers for debug
        Self {
            board: RefCell::new(gen_board(height, width, start, end)),
            solved_path: RefCell::new(Vec::new()),
            start: Node {
                x: start.0,
                y: start.1,
            },
            end: Node { x: end.0, y: end.1 },
            neighbours_list: RefCell::new(Vec::new()),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    /// x,height, column
    x: usize,
    /// y, width, row
    y: usize,
}

// A Node represented as a x and y coordinate in the planes of the game world
impl Node {
    /// get the cost from the end node to node x
    pub fn get_h_cost(self, node: Node) -> i32 {
        // manhatten metrik
        // let mut cost: i32 = 0;
        // let up_down_diff = (node.x as i32 - self.x as i32).abs();
        // let left_right_diff = (node.y as i32 - self.y as i32).abs();
        // let sum_of_moves = (up_down_diff + left_right_diff).abs();
        // cost = sum_of_moves * *BASE_G_COST;

        // diag faster
        let mut cost: i32 = 0;
        let ud = (node.x as i32 - self.x as i32).abs();
        let lr = (node.y as i32 - self.y as i32).abs();
        let mut sum_of_moves = (ud + lr).abs();
        if lr < ud {
            for _ in 0..lr {
                cost = cost + ((1 as f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves = sum_of_moves - 2;
            }
        }
        if lr > ud {
            for _ in 0..ud {
                cost = cost + ((1 as f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves = sum_of_moves - 2;
            }
        }
        cost = cost + (sum_of_moves * *BASE_G_COST);
        return cost;
    }
    /// get the cost from start to node x
    pub fn get_g_cost(self, node: Node) -> i32 {
        let mut cost: i32 = 0;
        let ud = (node.x as i32 - self.x as i32).abs();
        let lr = (node.y as i32 - self.y as i32).abs();
        let mut sum_of_moves = (ud + lr).abs();
        if lr < ud {
            for _ in 0..lr {
                cost = cost + ((1 as f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves = sum_of_moves - 2;
            }
        }
        if lr > ud {
            for _ in 0..ud {
                cost = cost + ((1 as f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves = sum_of_moves - 2;
            }
        }
        cost = cost + (sum_of_moves * *BASE_G_COST);
        return cost;
    }
    /// get the f cost g + h cost
    pub fn get_f_cost(&self, start: Node, end: Node) -> i32 {
        return self.clone().get_g_cost(start) + self.clone().get_h_cost(end);
    }
}

impl AStar {
    /// solve the board
    pub fn solve(&mut self) -> () {
        let board = self.board.clone().into_inner();
        let height = board.clone().len();
        let width = board[0].clone().len();
        let mut pos = get_rand_pos(height, width);

        // debug
        while pos == (self.end.x, self.end.y) || pos == (self.start.x, self.start.y) {
            pos = get_rand_pos(height, width);
        }

        let node = Node { x: pos.0, y: pos.1 };
        let h_cost = node.clone().get_h_cost(self.end.clone());
        let g_cost = node.clone().get_g_cost(self.start.clone());
        let f_cost = node
            .clone()
            .get_f_cost(self.start.clone(), self.end.clone());
        self.gen_surrounding();
        draw_board(board.clone(), vec![(pos.0, pos.1)]);
        println!("h {}, g {}, f {}", h_cost, g_cost, f_cost);
    }

    pub fn gen_surrounding(&mut self) {
        for _ in -1..1 {
            for _ in -1..1 {
                self.neighbours_list.borrow_mut().push(Node { x: 0, y: 0 });
            }
        }
    }
}

fn main() {
    let mut a_star = AStar::default();
    a_star.solve();
}
