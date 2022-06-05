use rand::{thread_rng, Rng};

fn get_random_num() -> usize {
    return thread_rng().gen_range(0..2);
}

fn gen_board(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = Vec::new();
    for y in 0..height {
        let mut new_vec: Vec<usize> = vec![];
        for x in 0..width {
            new_vec.push(get_random_num());
        }
        result.push(new_vec);
    }
    return result;
}

fn main() {
    let board = gen_board(5, 10);
    println!("{:?}", board);
    println!("Hello, world!");
}
