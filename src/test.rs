use crate::astar::*;

#[cfg(test)]
mod tests {
    use crate::astar::Node;

    #[test]
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
