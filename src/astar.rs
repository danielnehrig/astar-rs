use std::borrow::Borrow;
use std::cell::RefCell;

use colored::Colorize;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};

lazy_static! {
    /// the bondues for a diag node
    static ref DIAG_BONUS: f32 = 1.4;
    /// base g cost
    static ref BASE_G_COST: i32 = 10;
    /// indicator for start position
    static ref START_INDICATOR: i32 = 8;
    /// indicator for end position
    static ref END_INDICATOR: i32 = 9;
    /// the max board size
    static ref MAX_BOARD_SIZE: i32 = 13;
    /// the blocked node value
    static ref BLOCKED_NODE: i32 = 1;
    /// the free node value
    static ref FREE_NODE: i32 = 0;
}

/// random generation of blocked nodes
pub fn gen_blockade() -> i32 {
    if thread_rng().gen_ratio(1, 7) {
        // 1 in 7 chance to get a blocked node
        *BLOCKED_NODE
    } else {
        *FREE_NODE
    }
}

pub fn gen_range(x: i32) -> i32 {
    thread_rng().gen_range(x..*MAX_BOARD_SIZE)
}

/// get a random position on the board
/// used to create start and end points
pub fn get_rand_pos(height: i32, width: i32) -> (i32, i32) {
    (
        thread_rng().gen_range(0..height),
        thread_rng().gen_range(0..width),
    )
}

/// generate a 2d board
pub fn gen_board(height: i32, width: i32, start: (i32, i32), end: (i32, i32)) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for _ in 0..height {
        let mut new_vec: Vec<i32> = vec![];
        for _ in 0..width {
            new_vec.push(gen_blockade());
        }
        result.push(new_vec);
    }
    result[start.0 as usize][start.1 as usize] = *START_INDICATOR; // start indicator
    result[end.0 as usize][end.1 as usize] = *END_INDICATOR; // end indicator
    result
}

/// draw the board in stdout
pub fn draw_board(board: Vec<Vec<i32>>, selected: Vec<(i32, i32)>) {
    print!("     ");
    for y in 0..board[0].len() {
        print!("{} ", y);
    }
    println!();
    for (i, x) in board.iter().enumerate() {
        if i < 10 {
            print!("{}  [ ", i);
        } else {
            print!("{} [ ", i);
        }
        for (j, y) in x.clone().into_iter().enumerate() {
            // draw end in red
            let mut is_selected = false;
            for (x1, y1) in &selected {
                if i as i32 == *x1 && j as i32 == *y1 {
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
            } else if y == 1 {
                print!("{} ", format!("{}", y).bold().bright_blue());
            } else {
                print!("{} ", y);
            }
        }
        print!("]");
        println!();
    }
}

#[derive(Debug)]
pub struct AStar {
    /// the start node
    pub start: Node,
    /// the end node
    pub end: Node,
    /// the game board
    pub board: RefCell<Vec<Vec<i32>>>,
    /// the list of the solved path
    pub solved_path: RefCell<Vec<Node>>,
    /// the current neighbours
    pub neighbours_list: RefCell<Vec<Node>>,
}

impl Default for AStar {
    fn default() -> Self {
        let height = gen_range(3); // board height
        let width = gen_range(5); // board width
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
pub struct Node {
    /// x,height, column
    pub x: i32,
    /// y, width, row
    pub y: i32,
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
        let ud = (node.x as i32).abs_diff(self.x as i32) as i32;
        let lr = (node.y as i32).abs_diff(self.y as i32) as i32;
        // if there are both 1 move to the node x return 14 because that means its diagonal
        // and the cases below don't cover if you only move one diagonally
        if ud == 1 && lr == 1 {
            return ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
        }

        let mut sum_of_moves = (ud + lr).abs();
        if lr < ud {
            for _ in 0..lr {
                cost += ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves -= 2;
            }
        }
        if lr > ud {
            for _ in 0..ud {
                cost += ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves -= 2;
            }
        }
        cost += sum_of_moves * *BASE_G_COST;
        cost
    }
    /// get the cost from start to node x
    pub fn get_g_cost(self, node: Node) -> i32 {
        let mut cost: i32 = 0;
        let ud = (node.x as i32).abs_diff(self.x as i32) as i32;
        let lr = (node.y as i32).abs_diff(self.y as i32) as i32;
        let mut sum_of_moves = (ud + lr).abs();
        if ud == 1 && lr == 1 {
            return ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
        }
        if lr < ud {
            for _ in 0..lr {
                cost += ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves -= 2;
            }
        }
        if lr > ud {
            for _ in 0..ud {
                cost += ((1_f32 * *BASE_G_COST as f32) * *DIAG_BONUS) as i32;
                sum_of_moves -= 2;
            }
        }
        cost += sum_of_moves * *BASE_G_COST;
        cost
    }
    /// get the f cost g + h cost
    pub fn get_f_cost(&self, start: Node, end: Node) -> i32 {
        self.clone().get_g_cost(start) + self.clone().get_h_cost(end)
    }
}

impl AStar {
    /// solve the board
    pub fn solve(&mut self) {
        let board = self.board.clone().into_inner();
        let height: i32 = board.len() as i32;
        let width: i32 = board[0].clone().len() as i32;
        let mut pos = get_rand_pos(height, width);

        // debug
        while pos == (self.end.x, self.end.y) || pos == (self.start.x, self.start.y) {
            pos = get_rand_pos(height, width);
        }

        let node = Node { x: pos.0, y: pos.1 };
        let h_cost = node.clone().get_h_cost(self.end.clone());
        let g_cost = node.clone().get_g_cost(self.start.clone());
        let f_cost = node.get_f_cost(self.start.clone(), self.end.clone());
        self.gen_surrounding();
        draw_board(board, vec![(pos.0, pos.1)]);
        println!("selected node: h {}, g {}, f {}", h_cost, g_cost, f_cost);
        println!(
            "{} {} {}",
            "selected".to_string().yellow(),
            "start".to_string().green(),
            "end".to_string().red()
        );
    }

    /// generate a list of relative nodes from the selected/start node to be checked next
    pub fn gen_surrounding(&mut self) {
        for x in -1_i32..2_i32 {
            for y in -1_i32..2_i32 {
                let start = self.start.borrow();
                let board_x = self.board.borrow().len() as i32 - 1;
                let board_y = self.board.borrow()[0].len() as i32 - 1;
                let r_x = start.x + x; // relative position from start/current node
                let r_y = start.y + y;
                // cases:
                // don't add start node
                // don't add nodes out of bounds
                // don't add nodes that are blocked
                if !(r_x == start.x && r_y == start.y)
                    && ((r_x >= 0 && r_x <= board_x) && (r_y >= 0 && r_y <= board_y))
                {
                    let current_node = self.board.borrow()[r_x as usize][r_y as usize];
                    if current_node != 1 {
                        self.neighbours_list.borrow_mut().push(Node {
                            x: start.x + x as i32,
                            y: start.y + y as i32,
                        });
                    }
                }
            }
        }
        println!("{:?}", self.neighbours_list.borrow());
    }
}
