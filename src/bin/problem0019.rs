const T: [u16; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

fn first_day_is_sunday(year: u16, month: u16) -> bool {
    ((year + year / 4 - year / 100 + year / 400 + T[(month - 1) as usize] + 1) % 7) == 0
}

fn main() {
    let count: usize = (1901..=2000)
        .map(|y| (1..=12).filter(|&m| first_day_is_sunday(y, m)).count())
        .sum();

    println!("{count}");
}