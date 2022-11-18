#[cfg(test)]
mod tests {
    use crate::astar::Node;
    use crate::astar::*;

    #[test]
    ///    0 1 2 3 4 5 6
    ///  0
    ///  1
    ///  2
    ///  3
    ///  4
    ///  5           e
    ///  6             x
    ///  use cases:
    ///  Diagnoal cost is 14 because
    ///  it'll reach its goal faster
    /// Expected move pattern
    /// [6][6] = 14 cost
    fn h_cost_1() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 6, y: 6 };
        let h = x.get_cost(end_node);
        assert!(h == 14);
    }

    /// should be 10 here because the node x is directly
    /// behind the end node
    /// ... 5 6
    /// .
    /// 5   e
    /// 6   x
    /// like this
    /// the dots represent 0 - 4 like in the first test
    /// for simplicity sake we'll shorten them for following tests
    /// Expected move pattern
    /// [6][5] = 10 cost
    #[test]
    fn h_cost_2() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 6, y: 5 };
        let h = x.get_cost(end_node);
        assert!(h == 10);
    }

    /// should be 30
    /// ... 5 6 7 8
    /// .
    /// 5   e
    /// 6
    /// 7
    /// 8   x
    /// Expected move pattern
    /// [6][5] = 10 cost
    /// [7][5] = 20 cost
    /// [8][5] = 30 cost
    #[test]
    fn h_cost_3() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 8, y: 5 };
        let h = x.get_cost(end_node);
        assert!(h == 30);
    }

    /// should be 35
    /// ... 5 6 7 8
    /// .
    /// 5   e
    /// 6
    /// 7
    /// 8     x
    /// Expected move pattern
    /// [6][6] = 14 cost
    /// [7][6] = 24 cost
    /// [8][6] = 34 cost
    #[test]
    fn h_cost_4() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 8, y: 6 };
        let h = x.get_cost(end_node);
        assert!(h == 34);
    }

    ///    0 1 2 3 4 5 6 7 8
    ///  0
    ///  1   s
    ///  2
    ///  3
    ///  4
    ///  5           e
    ///  6
    ///  7
    ///  8             x
    ///  g cost should be 90
    ///  h cost should be 34
    ///  f cost should be g + h cost
    #[test]
    fn f_cost() {
        let start_node = Node { x: 1, y: 1 };
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 8, y: 6 };
        let f = x.get_f_cost(start_node, end_node);
        assert!(f == 90 + 34);
    }

    /// test serrounding neighbours
    #[test]
    fn neighbours() {
        let start = Node { x: 1, y: 1 };
        let end = Node { x: 2, y: 2 };
        let mut board = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        // set end and start
        board[start.x as usize][start.y as usize] = 8;
        board[end.x as usize][end.y as usize] = 9;
        // get astar with board
        let mut astar = AStar::with_board(board, start, end);
        // generate surrounding neighbours
        astar.gen_surrounding();
        assert!(astar.neighbours_list.borrow().len() == 6);
    }

    #[test]
    fn neighbours_2() {
        let start = Node { x: 1, y: 2 };
        let end = Node { x: 3, y: 4 };
        let mut board = vec![
            vec![0, 1, 1, 1, 1],
            vec![0, 1, 8, 1, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 9],
        ];
        // set end and start
        board[start.x as usize][start.y as usize] = 8;
        board[end.x as usize][end.y as usize] = 9;
        // get astar with board
        let mut astar = AStar::with_board(board, start, end);
        // generate surrounding neighbours
        astar.gen_surrounding();
        assert!(astar.neighbours_list.borrow().len() == 1);
    }
}
