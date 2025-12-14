fn main() {
    let mut number: u64 = 600851475143;
    let max_factor = (number as f64).sqrt() as u64;
    let mut factor = 3;
    let mut max_prime_factor = 0;

    while factor < max_factor {
        if number % factor == 0 {
            max_prime_factor = factor;
            number /= factor;

            while number % factor == 0 {
                number /= factor;
            }
        }

        factor += 2
    }

    println!("{max_prime_factor}")
}
