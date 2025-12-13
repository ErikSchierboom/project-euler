use itertools::Itertools;

fn main() {
    let sum_of_multiples: u32 = (3..1000).step_by(3)
        .chain((5..1000).step_by(5))
        .unique()
        .sum();

    println!("{sum_of_multiples}")
}