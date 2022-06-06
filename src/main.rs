mod astar;
mod test;
use astar::AStar;

fn main() {
    let mut a_star = AStar::default();
    a_star.solve();
}
