mod astar;
#[cfg(test)]
mod test;
use std::error::Error;

use astar::AStar;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a_star = AStar::default();
    a_star.solve();
    Ok(())
}
