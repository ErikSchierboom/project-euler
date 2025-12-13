fn main() {
    let number: i32 = 100;
    let square_of_sum = (number * (number + 1) / 2).pow(2);
    let sum_of_squares = (number * (number + 1) * (2 * number + 1)) / 6;
    let difference_of_squares = square_of_sum - sum_of_squares;

    println!("{difference_of_squares}")
}

