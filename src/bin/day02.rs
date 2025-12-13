fn main() {
    let fibonacci_sum: u32 = fibonacci()
        .filter(|fib| fib % 2 == 0)
        .take_while(|fib| fib < &4_000_000)
        .sum();

    println!("{fibonacci_sum}")
}

fn fibonacci() -> impl Iterator<Item = u32> {
    Fibonacci { prev_prev: 0, prev: 1 }
}

struct Fibonacci { prev_prev: u32, prev: u32 }


impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.prev_prev + self.prev;
        self.prev_prev = self.prev;
        self.prev = next;
        Some(next)
    }
}
