use crate::astar::*;

#[cfg(test)]
mod tests {
    use crate::astar::Node;

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
    fn h_cost_1() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 6, y: 6 };
        let h = x.get_h_cost(end_node);
        assert!(h == 14);
    }
    #[test]
    fn h_cost_2() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 6, y: 5 };
        let h = x.get_h_cost(end_node);
        assert!(h == 10);
    }
    #[test]
    fn h_cost_3() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 8, y: 5 };
        let h = x.get_h_cost(end_node);
        assert!(h == 30);
    }
    #[test]
    fn h_cost_4() {
        let end_node = Node { x: 5, y: 5 };
        let x = Node { x: 8, y: 6 };
        let h = x.get_h_cost(end_node);
        assert!(h == 34);
    }
}
